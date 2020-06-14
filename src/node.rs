#[allow(unused)]
pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span, warn,
    warn_span,
};

use std::hash::{Hash, Hasher};

use crate::{core::Level, universe::Id};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    // always level 0
    Leaf(Leaf),
    // Node::Inner can never have level 0
    Inode(Inode),
}

#[derive(Debug, Clone)]
pub struct Inode {
    pub level: Level,
    pub population: u32,
    pub result: Option<Id>,
    pub nw: Id,
    pub ne: Id,
    pub sw: Id,
    pub se: Id,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cell {
    Dead = 0u8,
    Alive = 1u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Leaf(pub Cell);

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

impl From<Leaf> for Node {
    fn from(cell: Leaf) -> Self {
        Node::Leaf(cell)
    }
}

impl From<Inode> for Node {
    fn from(inode: Inode) -> Self {
        Node::Inode(inode)
    }
}

impl Leaf {
    pub fn new(cell: Cell) -> Self {
        Self(cell)
    }

    pub fn alive(&self) -> bool {
        match self.0 {
            Cell::Dead => false,
            Cell::Alive => true,
        }
    }
}

impl Node {
    #[inline(always)]
    pub fn population(&self) -> u32 {
        match *self {
            Node::Inode(ref i) => i.population,
            Node::Leaf(c) => c.0 as u32,
        }
    }

    #[inline(always)]
    pub fn level(&self) -> Level {
        match *self {
            Node::Inode(ref i) => i.level,
            Node::Leaf(_) => Level::LEAF_LEVEL,
        }
    }
}
