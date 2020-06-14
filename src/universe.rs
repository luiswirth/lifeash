use std::collections::HashMap;

use crate::{
    core::{Level, Position, Quadrant::*},
    node::{Cell, Inode, Leaf, Node},
};

pub struct Universe {
    table: HashMap<Id, Node>,
    root: Option<Id>,
    generation: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub usize);

impl Id {
    fn node(self, univ: &Universe) -> &Node {
        univ.table.get(&self).unwrap()
    }

    #[allow(dead_code)]
    fn leaf(self, univ: &Universe) -> &Leaf {
        if let Node::Leaf(leaf) = self.node(univ) {
            leaf
        } else {
            panic!("not a leaf")
        }
    }

    fn inode(self, univ: &Universe) -> &Inode {
        if let Node::Inode(inode) = self.node(univ) {
            inode
        } else {
            panic!("not an inode")
        }
    }
}

impl Universe {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            root: None,
            generation: 0,
        }
    }

    pub fn initalize(&mut self) {
        self.root = Some(self.new_empty_tree(Level(3)));
    }
}

impl Universe {
    fn get_id(&mut self, node: Node) -> Id {
        if let Some(id) = self
            .table
            .iter()
            .find_map(|(i, n)| if *n == node { Some(i) } else { None })
            .copied()
        {
            id
        } else {
            let id = Id(self.table.len());
            self.table.insert(id, node);
            id
        }
    }

    pub fn new_leaf(&mut self, cell: Cell) -> Id {
        let node = Node::Leaf(Leaf(cell));
        self.get_id(node)
    }

    fn new_inode(&mut self, nwx: Id, nex: Id, swx: Id, sex: Id) -> Id {
        let childs = (
            nwx.node(self),
            nex.node(self),
            swx.node(self),
            sex.node(self),
        );
        let inode = match childs {
            (Node::Inode(nw), Node::Inode(ne), Node::Inode(sw), Node::Inode(se)) => {
                debug_assert!(nw.level == ne.level && ne.level == sw.level && sw.level == se.level);
                Inode {
                    level: nw.level + 1,
                    population: nw.population + ne.population + sw.population + se.population,
                    result: None,
                    nw: nwx,
                    ne: nex,
                    sw: swx,
                    se: sex,
                }
            }
            (Node::Leaf(nw), Node::Leaf(ne), Node::Leaf(sw), Node::Leaf(se)) => Inode {
                level: Level(1),
                population: [nw, ne, sw, se]
                    .iter()
                    .filter(|c| matches!(c.0, Cell::Alive))
                    .count() as u32,
                result: None,
                nw: nwx,
                ne: nex,
                sw: swx,
                se: sex,
            },
            _ => unreachable!(),
        };

        self.get_id(Node::Inode(inode))
    }

    fn new_empty_tree(&mut self, level: Level) -> Id {
        if level == Level::LEAF_LEVEL {
            self.new_leaf(Cell::Dead)
        } else {
            let child = Self::new_empty_tree(self, level - 1);
            self.new_inode(child, child, child, child)
        }
    }
}

impl Universe {
    pub fn get_tree_cell(&self, tree: Id, pos: impl Into<Position>) -> Cell {
        let pos = pos.into();
        match *tree.node(self) {
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

    fn set_tree_cell(&mut self, tree: Id, pos: impl Into<Position>, state: Cell) -> Id {
        let pos = pos.into();

        match *tree.node(self) {
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
                NorthWest => {
                    let nw = self.set_tree_cell(
                        nw,
                        pos.relative_to(level.quadrant_center(NorthWest)),
                        state,
                    );
                    self.new_inode(nw, ne, sw, se)
                }
                NorthEast => {
                    let ne = self.set_tree_cell(
                        ne,
                        pos.relative_to(level.quadrant_center(NorthEast)),
                        state,
                    );
                    self.new_inode(nw, ne, sw, se)
                }
                SouthWest => {
                    let sw = self.set_tree_cell(
                        sw,
                        pos.relative_to(level.quadrant_center(SouthWest)),
                        state,
                    );
                    self.new_inode(nw, ne, sw, se)
                }
                SouthEast => {
                    let se = self.set_tree_cell(
                        se,
                        pos.relative_to(level.quadrant_center(SouthEast)),
                        state,
                    );
                    self.new_inode(nw, ne, sw, se)
                }
            },
        }
    }
}

impl Universe {
    pub fn expand(&mut self) {
        let level = self.root.unwrap().inode(self).level;
        let border = self.new_empty_tree(level - 1);
        let (root_nw, root_ne, root_sw, root_se) = {
            let root = self.root.unwrap().inode(self);
            (root.nw, root.ne, root.sw, root.se)
        };
        let (nw, ne, sw, se) = (
            self.new_inode(border, border, border, root_nw),
            self.new_inode(border, border, root_ne, border),
            self.new_inode(border, root_sw, border, border),
            self.new_inode(root_se, border, border, border),
        );
        self.root = Some(self.new_inode(nw, ne, sw, se));
    }

    // since recursive make second function which always calls on root
    pub fn evolve_tree(&mut self, tree: Id) -> Id {
        {
            let inode = tree.inode(self);
            debug_assert!(inode.level >= Level(2), "must be level 2 or higher");
        }

        if let Some(result) = tree.inode(self).result {
            return result;
        }

        if tree.inode(self).level == 2 {
            self.manual_evolve(tree)
        } else {
            let (tree_nw, tree_ne, tree_sw, tree_se) = {
                let inode = tree.inode(self);
                (inode.nw, inode.ne, inode.sw, inode.se)
            };
            let n00 = self.centered_sub(tree_nw);
            let n01 = self.centered_horizontal(tree_nw, tree_ne);
            let n02 = self.centered_sub(tree_ne);
            let n10 = self.centered_vertical(tree_nw, tree_sw);
            let n11 = self.centered_subsub(tree);
            let n12 = self.centered_vertical(tree_ne, tree_se);
            let n20 = self.centered_sub(tree_sw);
            let n21 = self.centered_horizontal(tree_sw, tree_se);
            let n22 = self.centered_sub(tree_se);

            let (nw, ne, sw, se) = {
                let nw = self.new_inode(n00, n01, n10, n11);
                let ne = self.new_inode(n01, n02, n11, n12);
                let sw = self.new_inode(n10, n11, n20, n21);
                let se = self.new_inode(n11, n12, n21, n22);
                (
                    self.evolve_tree(nw),
                    self.evolve_tree(ne),
                    self.evolve_tree(sw),
                    self.evolve_tree(se),
                )
            };
            let result = self.new_inode(nw, ne, sw, se);

            // TODO have mutable way of changing this
            if let Node::Inode(inode) = self.table.get_mut(&tree).unwrap() {
                inode.result = Some(result);
            }

            result
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

    fn manual_evolve(&mut self, node: Id) -> Id {
        let inode = node.inode(self);
        debug_assert!(
            inode.level == 2,
            "manual evolution only at level 2 possible"
        );

        let mut all_bits: u16 = 0;
        for y in -2..2 {
            for x in -2..2 {
                all_bits = (all_bits << 1) + self.get_tree_cell(node, (x, y)) as u16;
            }
        }
        let (nw, ne, sw, se) = (
            self.one_gen(all_bits >> 5),
            self.one_gen(all_bits >> 4),
            self.one_gen(all_bits >> 1),
            self.one_gen(all_bits),
        );

        self.new_inode(nw, ne, sw, se)
    }

    // to update a cell be have to look at 9 cells (itself and the 8 directly adjecent ones)
    // so we still have to use a u16 bitmap.

    // the bottom three bits a..=c are the south neighbors
    // bits e..=g are the current row with 5 being the cell itself
    // i..=k are the north neighbors

    #[allow(clippy::inconsistent_digit_grouping)]
    fn one_gen(&mut self, mut bitmask: u16) -> Id {
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
    pub fn centered_horizontal(&mut self, west: Id, east: Id) -> Id {
        let (west, east) = (west.inode(self), east.inode(self));
        debug_assert!(west.level == east.level, "levels must be the same");

        let (nw, ne, sw, se) = (
            west.ne.inode(self).se,
            east.nw.inode(self).sw,
            west.se.inode(self).ne,
            east.sw.inode(self).nw,
        );
        self.new_inode(nw, ne, sw, se)
    }

    pub fn centered_vertical(&mut self, north: Id, south: Id) -> Id {
        let (north, south) = (north.inode(self), south.inode(self));
        debug_assert!(north.level == south.level, "levels must be the same");

        let (nw, ne, sw, se) = (
            north.sw.inode(self).se,
            north.se.inode(self).sw,
            south.nw.inode(self).ne,
            south.ne.inode(self).nw,
        );
        self.new_inode(nw, ne, sw, se)
    }

    pub fn centered_sub(&mut self, node: Id) -> Id {
        let node = node.inode(self);

        let (nw, ne, sw, se) = (
            node.nw.inode(self).se,
            node.ne.inode(self).sw,
            node.sw.inode(self).ne,
            node.se.inode(self).nw,
        );
        self.new_inode(nw, ne, sw, se)
    }

    pub fn centered_subsub(&mut self, node: Id) -> Id {
        let node = node.inode(self);
        let (nw, ne, sw, se) = (
            node.nw.inode(self).se.inode(self).se,
            node.ne.inode(self).sw.inode(self).sw,
            node.sw.inode(self).ne.inode(self).ne,
            node.se.inode(self).nw.inode(self).nw,
        );
        self.new_inode(nw, ne, sw, se)
    }
}

// old universe interface
// TODO: refactor (maybe make this a store module and put this in a "new" universe module)

impl Universe {
    pub fn set_cell(&mut self, pos: impl Into<Position>, cell: Cell) {
        let pos = pos.into();

        loop {
            let level = self.root.unwrap().node(self).level();
            if pos.in_bounds(level) {
                break;
            }
            self.expand();
        }

        self.root = Some(self.set_tree_cell(self.root.unwrap(), pos, cell));
    }

    pub fn get_cell(&self, pos: impl Into<Position>) -> Cell {
        let root = self.root.unwrap();
        self.get_tree_cell(root, pos)
    }

    pub fn evolve(&mut self) {
        loop {
            let iroot = self.root.unwrap().inode(self);
            let (nw_pop, ne_pop, sw_pop, se_pop) = (
                iroot.nw.node(self).population(),
                iroot.ne.node(self).population(),
                iroot.sw.node(self).population(),
                iroot.se.node(self).population(),
            );

            let (nw_inner_pop, ne_inner_pop, sw_inner_pop, se_inner_pop) = (
                iroot
                    .nw
                    .inode(self)
                    .se
                    .inode(self)
                    .se
                    .node(self)
                    .population(),
                iroot
                    .ne
                    .inode(self)
                    .sw
                    .inode(self)
                    .sw
                    .node(self)
                    .population(),
                iroot
                    .sw
                    .inode(self)
                    .ne
                    .inode(self)
                    .ne
                    .node(self)
                    .population(),
                iroot
                    .se
                    .inode(self)
                    .nw
                    .inode(self)
                    .nw
                    .node(self)
                    .population(),
            );

            if self.root.unwrap().node(self).level() >= 3
                && nw_pop == nw_inner_pop
                && ne_pop == ne_inner_pop
                && sw_pop == sw_inner_pop
                && se_pop == se_inner_pop
            {
                break;
            }
            self.expand();
        }

        let root = self.root.unwrap();

        self.root = Some(self.evolve_tree(root));
        self.generation += 1;
    }
}
