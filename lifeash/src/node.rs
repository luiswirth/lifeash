use std::hash::{Hash, Hasher};

use crate::{
    core::{Cell, Level},
    universe::Id,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Node {
    // always level 0
    Leaf(Leaf),
    // Node::Inner can never have level 0
    Inode(Inode),
}

#[derive(Debug, Clone)]
pub(crate) struct Inode {
    pub(crate) level: Level,
    pub(crate) population: u32,
    pub(crate) result: Option<Id>,
    pub(crate) nw: Id,
    pub(crate) ne: Id,
    pub(crate) sw: Id,
    pub(crate) se: Id,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Leaf(pub(crate) Cell);

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
    pub(crate) fn new(cell: Cell) -> Self {
        Self(cell)
    }

    #[allow(dead_code)]
    fn alive(self) -> bool {
        match self.0 {
            Cell::Dead => false,
            Cell::Alive => true,
        }
    }
}

impl Node {
    #[inline(always)]
    pub(crate) fn population(&self) -> u32 {
        match *self {
            Node::Inode(ref i) => i.population,
            Node::Leaf(c) => c.0 as u32,
        }
    }

    #[inline(always)]
    pub(crate) fn level(&self) -> Level {
        match *self {
            Node::Inode(ref i) => i.level,
            Node::Leaf(_) => Level::LEAF_LEVEL,
        }
    }
}
