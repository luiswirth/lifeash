use std::collections::HashSet;

use crate::{
    core::{Level, Position, Quadrant::*},
    node::{Cell, Inode, Leaf, Node},
};

pub struct Universe {
    table: HashSet<Node>,
    root: Option<&'static Node>,
    generation: usize,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            table: HashSet::new(),
            root: None,
            generation: 0,
        }
    }

    pub fn initalize(&mut self) {
        self.root = Some(self.new_empty_tree(Level(3)));
    }
}

impl Universe {
    pub fn get_tree_cell(&self, tree: &Node, pos: impl Into<Position>) -> Cell {
        let pos = pos.into();
        match *tree {
            Node::Leaf(c) => c.0,
            Node::Inode(Inode {
                level,
                population: _,
                result: _,
                nw,
                ne,
                sw,
                se,
            }) => match pos.quadrant() {
                NorthWest => {
                    self.get_tree_cell(nw, pos.relative_to(level.quadrant_center(NorthWest)))
                }
                NorthEast => {
                    self.get_tree_cell(ne, pos.relative_to(level.quadrant_center(NorthEast)))
                }
                SouthWest => {
                    self.get_tree_cell(sw, pos.relative_to(level.quadrant_center(SouthWest)))
                }
                SouthEast => {
                    self.get_tree_cell(se, pos.relative_to(level.quadrant_center(SouthEast)))
                }
            },
        }
    }

    fn set_tree_cell(&mut self, tree: &Node, pos: impl Into<Position>, state: Cell) -> &Node {
        let pos = pos.into();

        match *tree {
            Node::Leaf(_) => self.new_leaf(state),
            Node::Inode(Inode {
                level,
                population: _,
                result: _,
                nw,
                ne,
                sw,
                se,
            }) => match pos.quadrant() {
                NorthWest => self.new_inode(
                    self.set_cell(nw, pos.relative_to(level.quadrant_center(NorthWest)), state),
                    ne,
                    sw,
                    se,
                ),

                NorthEast => self.new_inode(
                    nw,
                    self.set_cell(ne, pos.relative_to(level.quadrant_center(NorthEast)), state),
                    sw,
                    se,
                ),
                SouthWest => self.new_inode(
                    nw,
                    ne,
                    self.set_cell(sw, pos.relative_to(level.quadrant_center(SouthWest)), state),
                    se,
                ),
                SouthEast => self.new_inode(
                    nw,
                    ne,
                    sw,
                    self.set_cell(se, pos.relative_to(level.quadrant_center(SouthEast)), state),
                ),
            },
        }
    }
}

impl Universe {
    // creation methods return references into hashset
    // other methods consume references and return new references into hashset

    // TODO: maybe return Leaf and Inode instead of Node
    pub fn new_leaf(&mut self, cell: Cell) -> &Node {
        let node = Node::Leaf(Leaf(cell));
        self.table.get_or_insert(node)
    }

    pub fn new_inode(&mut self, nw: &Node, ne: &Node, sw: &Node, se: &Node) -> &Node {
        let inode = match (*nw, *ne, *sw, *se) {
            (Node::Inode(nw), Node::Inode(ne), Node::Inode(sw), Node::Inode(se)) => {
                debug_assert!(nw.level == ne.level && ne.level == sw.level && sw.level == se.level);
                Inode {
                    level: nw.level + 1,
                    population: nw.population + ne.population + sw.population + se.population,
                    result: None,
                    nw: &nw.into(),
                    ne: &ne.into(),
                    sw: &sw.into(),
                    se: &se.into(),
                }
            }
            (Node::Leaf(nw), Node::Leaf(ne), Node::Leaf(sw), Node::Leaf(se)) => Inode {
                level: Level(1),
                population: [nw, ne, sw, se]
                    .iter()
                    .filter(|c| matches!(c.0, Cell::Alive))
                    .count() as u32,
                result: None,
                nw: &nw.into(),
                ne: &ne.into(),
                sw: &sw.into(),
                se: &se.into(),
            },
            _ => unreachable!(),
        };

        self.table.get_or_insert(Node::Inode(inode))
    }

    pub fn new_empty_tree(&mut self, level: Level) -> &Node {
        if level == Level::LEAF_LEVEL {
            self.new_leaf(Cell::Dead)
        } else {
            let child = Self::new_empty_tree(self, level - 1);
            self.new_inode(child, child, child, child)
        }
    }
}

impl Universe {
    pub fn expand(&mut self) {
        let root = self.root.unwrap().inode();
        let border = self.new_empty_tree(root.level - 1);
        self.root = Some(self.new_inode(
            self.new_inode(border, border, border, root.nw),
            self.new_inode(border, border, root.ne, border),
            self.new_inode(border, root.sw, border, border),
            self.new_inode(root.se, border, border, border),
        ));
    }

    // since recursive make second function which always calls on root
    pub fn evolve_tree(&mut self, tree: &Node) -> &Node {
        let inode = tree.inode_ref();
        debug_assert!(inode.level >= Level(2), "must be level 2 or higher");

        if let Some(result) = inode.result {
            return result;
        }

        if inode.level == 2 {
            self.manual_evolve(tree)
        } else {
            let n00 = self.centered_sub(inode.nw.inode_ref());
            let n01 = self.centered_horizontal(inode.nw.inode_ref(), inode.ne.inode_ref());
            let n02 = self.centered_sub(inode.ne.inode_ref());
            let n10 = self.centered_vertical(inode.nw.inode_ref(), inode.sw.inode_ref());
            let n11 = self.centered_subsub(inode);
            let n12 = self.centered_vertical(inode.ne.inode_ref(), inode.se.inode_ref());
            let n20 = self.centered_sub(inode.sw.inode_ref());
            let n21 = self.centered_horizontal(inode.sw.inode_ref(), inode.se.inode_ref());
            let n22 = self.centered_sub(inode.se.inode_ref());

            inode.result = Some(
                self.new_inode(
                    self.evolve_tree(self.new_inode(
                        n00.into(),
                        n01.into(),
                        n10.into(),
                        n11.into(),
                    ))
                    .into(),
                    self.evolve_tree(self.new_inode(
                        n01.into(),
                        n02.into(),
                        n11.into(),
                        n12.into(),
                    ))
                    .into(),
                    self.evolve_tree(self.new_inode(
                        n10.into(),
                        n11.into(),
                        n20.into(),
                        n21.into(),
                    ))
                    .into(),
                    self.evolve_tree(self.new_inode(
                        n11.into(),
                        n12.into(),
                        n21.into(),
                        n22.into(),
                    ))
                    .into(),
                ),
            );

            inode.result.unwrap()
        }
    }

    // Inode at level 2 contains 16 cells
    // these can be represented by a bitmap of u16
    // p is at position (-2, -2) and a at (1, 1)

    // p o n m
    // l k j i
    // h g f e
    // d c b a
    // 0b_ponm_lkji_hgfe_dcba

    fn manual_evolve(&mut self, node: &Node) -> &Node {
        let inode = node.inode_ref();
        debug_assert!(
            inode.level == 2,
            "manual evolution only at level 2 possible"
        );

        let mut all_bits: u16 = 0;
        for y in -2..2 {
            for x in -2..2 {
                all_bits = (all_bits << 1) + self.get_tree_cell(self.root.unwrap(), (x, y)) as u16;
            }
        }
        self.new_inode(
            self.one_gen(all_bits >> 5), // nw
            self.one_gen(all_bits >> 4), // ne
            self.one_gen(all_bits >> 1), // sw
            self.one_gen(all_bits),      // se
        )
    }

    // to update a cell be have to look at 9 cells (itself and the 8 directly adjecent ones)
    // so we still have to use a u16 bitmap.

    // the bottom three bits a..=c are the south neighbors
    // bits e..=g are the current row with 5 being the cell itself
    // i..=k are the north neighbors

    #[allow(clippy::inconsistent_digit_grouping)]
    fn one_gen(&mut self, mut bitmask: u16) -> &Node {
        if bitmask == 0 {
            return self.new_leaf(Cell::Dead);
        }

        let center = (bitmask >> 5) & 1;
        bitmask &= 0b00000__111_0101_0111; // mask out bits we don't care about
        let neighbor_count = bitmask.count_ones();
        if neighbor_count == 3 || (neighbor_count == 2 && center != 0) {
            self.new_leaf(Cell::Alive)
        } else {
            self.new_leaf(Cell::Dead)
        }
    }
}

// this can move into another class, when the refactoring of the leaves to Bool8x8 has been done.
impl Universe {
    pub fn centered_horizontal(&mut self, west: &Inode, east: &Inode) -> &Node {
        debug_assert!(west.level == east.level, "levels must be the same");

        self.new_inode(
            west.ne.inode_ref().se,
            east.nw.inode_ref().sw,
            west.se.inode_ref().ne,
            east.sw.inode_ref().nw,
        )
    }

    pub fn centered_vertical(&mut self, north: &Inode, south: &Inode) -> &Node {
        debug_assert!(north.level == south.level, "levels must be the same");

        self.new_inode(
            north.sw.inode_ref().se,
            north.se.inode_ref().sw,
            south.nw.inode_ref().ne,
            south.ne.inode_ref().nw,
        )
    }

    pub fn centered_sub(&mut self, node: &Inode) -> &Node {
        self.new_inode(
            node.nw.inode_ref().se,
            node.ne.inode_ref().sw,
            node.sw.inode_ref().ne,
            node.se.inode_ref().nw,
        )
    }

    pub fn centered_subsub(&mut self, node: &Inode) -> &Node {
        self.new_inode(
            node.nw.inode_ref().se.inode_ref().se,
            node.ne.inode_ref().sw.inode_ref().sw,
            node.sw.inode_ref().ne.inode_ref().ne,
            node.se.inode_ref().nw.inode_ref().nw,
        )
    }
}

// old universe interface
// TODO: refactor (maybe make this a store module and put this in a "new" universe module)

impl Universe {
    pub fn set_cell(&mut self, pos: impl Into<Position>, cell: Cell) {
        let pos = pos.into();
        let root = self.root.unwrap();

        loop {
            if pos.in_bounds(root.level()) {
                break;
            }
            self.expand();
        }

        self.root = Some(self.set_tree_cell(root, pos, cell));
    }

    pub fn get_cell(&mut self, pos: impl Into<Position>) -> Cell {
        let root = self.root.unwrap();
        self.get_tree_cell(root, pos)
    }

    pub fn evolve(&mut self) {
        let root = self.root.unwrap();
        while root.level() < 3
            || root.inode_ref().nw.population()
                != root
                    .inode_ref()
                    .nw
                    .inode_ref()
                    .se
                    .inode_ref()
                    .se
                    .population()
            || root.inode_ref().ne.population()
                != root
                    .inode_ref()
                    .ne
                    .inode_ref()
                    .sw
                    .inode_ref()
                    .sw
                    .population()
            || root.inode_ref().sw.population()
                != root
                    .inode_ref()
                    .sw
                    .inode_ref()
                    .ne
                    .inode_ref()
                    .ne
                    .population()
            || root.inode_ref().se.population()
                != root
                    .inode_ref()
                    .se
                    .inode_ref()
                    .nw
                    .inode_ref()
                    .nw
                    .population()
        {
            self.expand()
        }
        let root = self.root.unwrap();

        self.root = Some(self.evolve_tree(root));
        self.generation += 1;
    }
}
