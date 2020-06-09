#[allow(unused)]
pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span, warn,
    warn_span,
};

use std::hash::{Hash, Hasher};

use crate::core::{Level, Position, Quadrant::*};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    // always level 0
    Leaf(Cell),
    // Node::Inner can never have level 0
    Inner(Inode),
}

#[derive(Debug, Clone)]
pub struct Inode {
    level: Level,
    population: u32,
    result: Option<Box<Inode>>,
    pub nw: Box<Node>,
    pub ne: Box<Node>,
    pub sw: Box<Node>,
    pub se: Box<Node>,
}

impl PartialEq for Inode {
    fn eq(&self, other: &Self) -> bool {
        self.nw == other.nw && self.ne == other.ne && self.sw == other.sw && self.se == other.se
    }
}
impl Eq for Inode {}

impl Hash for Inode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nw.hash(state);
        self.ne.hash(state);
        self.sw.hash(state);
        self.se.hash(state);
    }
}

// reduce to bit
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cell {
    Dead = 0u8,
    Alive = 1u8,
}

impl From<Cell> for Node {
    fn from(cell: Cell) -> Self {
        Node::Leaf(cell)
    }
}

impl From<Cell> for Box<Node> {
    fn from(cell: Cell) -> Self {
        Box::new(cell.into())
    }
}

impl From<Inode> for Node {
    fn from(inode: Inode) -> Self {
        Node::Inner(inode)
    }
}

impl From<Inode> for Box<Node> {
    fn from(inode: Inode) -> Self {
        Box::new(inode.into())
    }
}

impl Cell {
    fn new(alive: bool) -> Self {
        if alive {
            Cell::Alive
        } else {
            Cell::Dead
        }
    }

    pub fn alive(&self) -> bool {
        match *self {
            Cell::Dead => false,
            Cell::Alive => true,
        }
    }
}

impl Inode {
    pub fn new(nw: Node, ne: Node, sw: Node, se: Node) -> Self {
        match (nw, ne, sw, se) {
            (Node::Inner(nw), Node::Inner(ne), Node::Inner(sw), Node::Inner(se)) => {
                debug_assert!(nw.level == ne.level && ne.level == sw.level && sw.level == se.level);
                Inode {
                    level: nw.level + 1,
                    population: nw.population + ne.population + sw.population + se.population,
                    result: None,
                    nw: nw.into(),
                    ne: ne.into(),
                    sw: sw.into(),
                    se: se.into(),
                }
            }
            (Node::Leaf(nw), Node::Leaf(ne), Node::Leaf(sw), Node::Leaf(se)) => Inode {
                level: Level(1),
                population: [nw, ne, sw, se]
                    .iter()
                    .filter(|c| matches!(c, Cell::Alive))
                    .count() as u32,
                result: None,
                nw: nw.into(),
                ne: ne.into(),
                sw: sw.into(),
                se: se.into(),
            },
            _ => unreachable!(),
        }
    }
}

impl Node {
    #[inline(always)]
    pub fn new_leaf(alive: bool) -> Self {
        Node::Leaf(Cell::new(alive))
    }

    #[inline(always)]
    pub fn new_inner(nw: Node, ne: Node, sw: Node, se: Node) -> Self {
        Node::Inner(Inode::new(nw, ne, sw, se))
    }

    #[allow(unused)]
    #[inline(always)]
    pub fn cell(self) -> Cell {
        if let Node::Leaf(cell) = self {
            cell
        } else {
            panic!("not a leaf")
        }
    }

    #[allow(unused)]
    #[inline(always)]
    pub fn cell_ref(&self) -> &Cell {
        if let Node::Leaf(ref cell) = self {
            cell
        } else {
            panic!("not a leaf")
        }
    }

    #[inline(always)]
    pub fn inode(self) -> Inode {
        if let Node::Inner(inode) = self {
            inode
        } else {
            panic!("not an inner")
        }
    }

    #[inline(always)]
    pub fn inode_ref(&self) -> &Inode {
        if let Node::Inner(ref inode) = self {
            inode
        } else {
            panic!("not an inner")
        }
    }

    #[inline(always)]
    pub fn population(&self) -> u32 {
        match *self {
            Node::Inner(ref i) => i.population,
            Node::Leaf(c) => c as u32,
        }
    }

    #[inline(always)]
    pub fn level(&self) -> Level {
        match *self {
            Node::Inner(ref i) => i.level,
            Node::Leaf(_) => Level::LEAF_LEVEL,
        }
    }
}

impl Node {
    pub fn new_empty_tree(level: Level) -> Self {
        if level == Level::LEAF_LEVEL {
            Self::new_leaf(false)
        } else {
            let child = Self::new_empty_tree(level - 1);
            Self::new_inner(child.clone(), child.clone(), child.clone(), child)
        }
    }
}

impl Node {
    pub fn get_bit(&self, pos: impl Into<Position>) -> u16 {
        let pos = pos.into();
        match *self {
            // independent of x and y
            Node::Leaf(c) => c as u16,
            Node::Inner(Inode {
                level,
                population: _,
                result: _,
                ref nw,
                ref ne,
                ref sw,
                ref se,
            }) => {
                // coordinates are not used at level 1, therefore the value doesn't matter. We default it to 0
                //let offset = if level >= 2 { 1 << (level - 2) } else { 0 };

                match pos.quadrant() {
                    NorthWest => nw.get_bit(pos.relative_to(level.quadrant_center(NorthWest))),
                    NorthEast => ne.get_bit(pos.relative_to(level.quadrant_center(NorthEast))),
                    SouthWest => sw.get_bit(pos.relative_to(level.quadrant_center(SouthWest))),
                    SouthEast => se.get_bit(pos.relative_to(level.quadrant_center(SouthEast))),
                }
            }
        }
    }

    pub fn set_bit(self, pos: impl Into<Position>, alive: bool) -> Self {
        let pos = pos.into();
        match self {
            // independent of x and y
            Node::Leaf(_) => Self::new_leaf(alive),
            Node::Inner(Inode {
                level,
                population: _,
                result: _,
                nw,
                ne,
                sw,
                se,
            }) => {
                // coordinates are not used at level 1, therefore the value doesn't matter. We default it to 0
                //let offset = if level >= 2 { 1 << (level - 2) } else { 0 };

                match pos.quadrant() {
                    NorthWest => Self::new_inner(
                        nw.set_bit(pos.relative_to(level.quadrant_center(NorthWest)), alive),
                        *ne,
                        *sw,
                        *se,
                    ),

                    NorthEast => Self::new_inner(
                        *nw,
                        ne.set_bit(pos.relative_to(level.quadrant_center(NorthEast)), alive),
                        *sw,
                        *se,
                    ),
                    SouthWest => Self::new_inner(
                        *nw,
                        *ne,
                        sw.set_bit(pos.relative_to(level.quadrant_center(SouthWest)), alive),
                        *se,
                    ),
                    SouthEast => Self::new_inner(
                        *nw,
                        *ne,
                        *sw,
                        se.set_bit(pos.relative_to(level.quadrant_center(SouthEast)), alive),
                    ),
                }
            }
        }
    }

    pub fn expand_universe(self) -> Self {
        let node = self.inode();
        let border = Self::new_empty_tree(node.level - 1);
        Self::new_inner(
            Self::new_inner(border.clone(), border.clone(), border.clone(), *node.nw),
            Self::new_inner(border.clone(), border.clone(), *node.ne, border.clone()),
            Self::new_inner(border.clone(), *node.sw, border.clone(), border.clone()),
            Self::new_inner(*node.se, border.clone(), border.clone(), border),
        )
    }
}

impl Inode {
    pub fn centered_sub(&self) -> Self {
        Self::new(
            *self.nw.inode_ref().se.clone(),
            *self.ne.inode_ref().sw.clone(),
            *self.sw.inode_ref().ne.clone(),
            *self.se.inode_ref().nw.clone(),
        )
    }

    pub fn centered_horizontal(west: &Self, east: &Self) -> Self {
        debug_assert!(west.level == east.level, "levels must be the same");

        Self::new(
            *west.ne.inode_ref().se.clone(),
            *east.nw.inode_ref().sw.clone(),
            *west.se.inode_ref().ne.clone(),
            *east.sw.inode_ref().nw.clone(),
        )
    }

    pub fn centered_vertical(north: &Self, south: &Self) -> Self {
        debug_assert!(north.level == south.level, "levels must be the same");

        Self::new(
            *north.sw.inode_ref().se.clone(),
            *north.se.inode_ref().sw.clone(),
            *south.nw.inode_ref().ne.clone(),
            *south.ne.inode_ref().nw.clone(),
        )
    }

    pub fn centered_subsub(&self) -> Self {
        Self::new(
            *self.nw.inode_ref().se.inode_ref().se.clone(),
            *self.ne.inode_ref().sw.inode_ref().sw.clone(),
            *self.sw.inode_ref().ne.inode_ref().ne.clone(),
            *self.se.inode_ref().nw.inode_ref().nw.clone(),
        )
    }

    pub fn next_generation(mut self) -> Self {
        debug_assert!(self.level >= Level(2), "must be level 2 or higher");

        if let Some(result) = self.result {
            return *result;
        }

        if self.level == 2 {
            Node::manual_simulation(self.into()).inode()
        } else {
            let n00 = self.nw.inode_ref().centered_sub();
            let n01 = Self::centered_horizontal(self.nw.inode_ref(), self.ne.inode_ref());
            let n02 = self.ne.inode_ref().centered_sub();
            let n10 = Self::centered_vertical(self.nw.inode_ref(), self.sw.inode_ref());
            let n11 = self.centered_subsub();
            let n12 = Self::centered_vertical(self.ne.inode_ref(), self.se.inode_ref());
            let n20 = self.sw.inode_ref().centered_sub();
            let n21 = Self::centered_horizontal(self.sw.inode_ref(), self.se.inode_ref());
            let n22 = self.se.inode_ref().centered_sub();

            self.result = Some(Box::new(Self::new(
                Self::new(
                    n00.into(),
                    n01.clone().into(),
                    n10.clone().into(),
                    n11.clone().into(),
                )
                .next_generation()
                .into(),
                Self::new(
                    n01.into(),
                    n02.into(),
                    n11.clone().into(),
                    n12.clone().into(),
                )
                .next_generation()
                .into(),
                Self::new(
                    n10.into(),
                    n11.clone().into(),
                    n20.into(),
                    n21.clone().into(),
                )
                .next_generation()
                .into(),
                Self::new(n11.into(), n12.into(), n21.into(), n22.into())
                    .next_generation()
                    .into(),
            )));

            *self.result.unwrap()
        }
    }
}

impl Node {
    // Inode at level 2 contains 16 cells
    // these can be represented by a bitmap of u16
    // p is at position (-2, -2) and a at (1, 1)

    // p o n m
    // l k j i
    // h g f e
    // d c b a
    // 0b_ponm_lkji_hgfe_dcba

    pub fn manual_simulation(self) -> Self {
        let inode = self.inode_ref();
        debug_assert!(inode.level == 2, "manual simulation only for level 2");

        let mut all_bits: u16 = 0;
        for y in -2..2 {
            for x in -2..2 {
                all_bits = (all_bits << 1) + self.get_bit((x, y));
            }
        }
        Self::new_inner(
            Self::one_gen(all_bits >> 5), // nw
            Self::one_gen(all_bits >> 4), // ne
            Self::one_gen(all_bits >> 1), // sw
            Self::one_gen(all_bits),      // se
        )
    }

    // to update a cell be have to look at 9 cells (itself and the 8 directly adjecent ones)
    // so we still have to use a u16 bitmap.

    // the bottom three bits a..=c are the south neighbors
    // bits e..=g are the current row with 5 being the cell itself
    // i..=k are the north neighbors

    #[allow(clippy::inconsistent_digit_grouping)]
    pub fn one_gen(mut bitmask: u16) -> Self {
        if bitmask == 0 {
            return Self::new_leaf(false);
        }

        let center = (bitmask >> 5) & 1;
        bitmask &= 0b00000__111_0101_0111; // mask out bits we don't care about (?)
        let neighbor_count = bitmask.count_ones();
        if neighbor_count == 3 || (neighbor_count == 2 && center != 0) {
            Self::new_leaf(true)
        } else {
            Self::new_leaf(false)
        }
    }
}
