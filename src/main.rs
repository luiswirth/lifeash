#[allow(unused)]
use color_eyre::{Help, Report, Result};
#[allow(unused)]
use eyre::{eyre, WrapErr};

#[allow(unused)]
use tracing::{
    debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span, warn,
    warn_span,
};

mod logging;

fn main() -> Result<()> {
    logging::setup();

    Ok(())
}

struct Universe {
    generation: usize,
    root: Node,
}

impl Universe {
    fn new() -> Self {
        Self {
            generation: 0,
            root: Node::new_empty_tree(3),
        }
    }

    fn set_bit(&mut self, x: isize, y: isize) {
        let mut copy = self.root.clone();
        loop {
            let max_coordinate: isize = 1 << (self.root.inode_ref().level - 1);
            if -max_coordinate <= x
                && x <= max_coordinate - 1
                && -max_coordinate <= y
                && y <= max_coordinate - 1
            {
                break;
            }
            copy = copy.expand_universe();
        }

        self.root = copy.set_bit(x, y);
    }

    fn run_step(&mut self) {
        while self.root.inode_ref().level < 3
            || self.root.inode_ref().nw.inode_ref().population
                != self
                    .root
                    .inode_ref()
                    .nw
                    .inode_ref()
                    .se
                    .inode_ref()
                    .se
                    .inode_ref()
                    .population
            || self.root.inode_ref().ne.inode_ref().population
                != self
                    .root
                    .inode_ref()
                    .ne
                    .inode_ref()
                    .sw
                    .inode_ref()
                    .sw
                    .inode_ref()
                    .population
            || self.root.inode_ref().sw.inode_ref().population
                != self
                    .root
                    .inode_ref()
                    .sw
                    .inode_ref()
                    .ne
                    .inode_ref()
                    .ne
                    .inode_ref()
                    .population
            || self.root.inode_ref().se.inode_ref().population
                != self
                    .root
                    .inode_ref()
                    .se
                    .inode_ref()
                    .nw
                    .inode_ref()
                    .nw
                    .inode_ref()
                    .population
        {
            self.root = self.root.clone().expand_universe();
        }

        self.root = self.root.clone().inode().next_generation().into();
        self.generation += 1;
    }
}

// root
// inner nodes / halfleaves /
// outer nodes / leaves

#[derive(Debug, Clone)]
pub enum Node {
    // always level 0
    Leaf(Cell),
    // Node::Inner can never have level 0
    Inner(Inode),
}

#[derive(Debug, Clone)]
pub struct Inode {
    pub level: usize,
    pub population: usize,
    pub nw: Box<Node>,
    pub ne: Box<Node>,
    pub sw: Box<Node>,
    pub se: Box<Node>,
}

// reduce to bit
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
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
}

impl Inode {
    fn new(nw: Node, ne: Node, sw: Node, se: Node) -> Self {
        match (nw, ne, sw, se) {
            (Node::Inner(nw), Node::Inner(ne), Node::Inner(sw), Node::Inner(se)) => {
                debug_assert!(nw.level == ne.level && ne.level == sw.level && sw.level == se.level);
                Inode {
                    level: nw.level + 1,
                    population: nw.population + ne.population + sw.population + se.population,
                    nw: nw.into(),
                    ne: ne.into(),
                    sw: sw.into(),
                    se: se.into(),
                }
            }
            (Node::Leaf(nw), Node::Leaf(ne), Node::Leaf(sw), Node::Leaf(se)) => Inode {
                level: 1,
                population: [nw, ne, sw, se]
                    .iter()
                    .filter(|c| matches!(c, Cell::Alive))
                    .count(),
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
    fn new_leaf(alive: bool) -> Self {
        Node::Leaf(Cell::new(alive))
    }

    #[inline(always)]
    fn new_inner(nw: Node, ne: Node, sw: Node, se: Node) -> Self {
        Node::Inner(Inode::new(nw, ne, sw, se))
    }

    #[inline(always)]
    fn cell(self) -> Cell {
        if let Node::Leaf(cell) = self {
            cell
        } else {
            panic!("not a leaf")
        }
    }

    #[inline(always)]
    fn cell_ref(&self) -> &Cell {
        if let Node::Leaf(ref cell) = self {
            cell
        } else {
            panic!("not a leaf")
        }
    }

    #[inline(always)]
    fn inode(self) -> Inode {
        if let Node::Inner(inode) = self {
            inode
        } else {
            panic!("not an inner")
        }
    }

    #[inline(always)]
    fn inode_ref(&self) -> &Inode {
        if let Node::Inner(ref inode) = self {
            inode
        } else {
            panic!("not an inner")
        }
    }
}

impl Node {
    fn new_empty_tree(level: usize) -> Self {
        if level == 0 {
            Self::new_leaf(false)
        } else {
            let child = Self::new_empty_tree(level - 1);
            Self::new_inner(child.clone(), child.clone(), child.clone(), child)
        }
    }
}

impl Node {
    // TODO: check and understand
    fn set_bit(self, x: isize, y: isize) -> Self {
        match self {
            Node::Leaf(_) => Self::new_leaf(true).into(),
            Node::Inner(Inode {
                level,
                population,
                nw,
                ne,
                sw,
                se,
            }) => {
                let offset = 1 << (level - 2);
                match (x < 0, y < 0) {
                    (true, true) => {
                        Self::new_inner(nw.set_bit(x + offset, y + offset), *ne, *sw, *se).into()
                    }
                    (true, false) => {
                        Self::new_inner(*nw, *ne, sw.set_bit(x + offset, y - offset), *se).into()
                    }
                    (false, true) => {
                        Self::new_inner(*nw, ne.set_bit(x - offset, y + offset), *sw, *se).into()
                    }
                    (false, false) => {
                        Self::new_inner(*nw, *ne, *sw, se.set_bit(x - offset, y - offset)).into()
                    }
                }
            }
        }
    }

    fn get_bit(&self, x: isize, y: isize) -> u16 {
        match self {
            Node::Leaf(c) => c.clone() as u16,
            Node::Inner(Inode {
                level,
                population,
                nw,
                ne,
                sw,
                se,
            }) => {
                let offset = 1 << (level - 2);
                match (x < 0, y < 0) {
                    (true, true) => nw.get_bit(x + offset, y + offset),
                    (true, false) => sw.get_bit(x + offset, y - offset),
                    (false, true) => ne.get_bit(x - offset, y + offset),
                    (false, false) => se.get_bit(x - offset, y - offset),
                }
            }
        }
    }

    fn expand_universe(self) -> Self {
        let node = self.inode();
        let border = Self::new_empty_tree(node.level - 1);
        Self::new_inner(
            Self::new_inner(border.clone(), border.clone(), border.clone(), *node.nw).into(),
            Self::new_inner(border.clone(), border.clone(), *node.ne, border.clone()).into(),
            Self::new_inner(border.clone(), *node.sw, border.clone(), border.clone()).into(),
            Self::new_inner(*node.se, border.clone(), border.clone(), border).into(),
        )
        .into()
    }
}

impl Inode {
    fn centered_sub(&self) -> Self {
        Self::new(
            *self.nw.inode_ref().se.clone(),
            *self.ne.inode_ref().sw.clone(),
            *self.sw.inode_ref().ne.clone(),
            *self.se.inode_ref().nw.clone(),
        )
    }

    fn centered_horizontal(west: &Self, east: &Self) -> Self {
        debug_assert!(west.level == east.level, "levels must be the same");

        Self::new(
            *west.ne.inode_ref().se.clone(),
            *east.nw.inode_ref().sw.clone(),
            *west.se.inode_ref().ne.clone(),
            *east.sw.inode_ref().nw.clone(),
        )
    }

    fn centered_vertical(north: &Self, south: &Self) -> Self {
        debug_assert!(north.level == south.level, "levels must be the same");

        Self::new(
            *north.sw.inode_ref().se.clone(),
            *north.se.inode_ref().sw.clone(),
            *south.sw.inode_ref().ne.clone(),
            *south.ne.inode_ref().nw.clone(),
        )
    }

    fn centered_subsub(&self) -> Self {
        Self::new(
            *self.nw.inode_ref().se.inode_ref().se.clone(),
            *self.ne.inode_ref().sw.inode_ref().sw.clone(),
            *self.sw.inode_ref().ne.inode_ref().ne.clone(),
            *self.se.inode_ref().nw.inode_ref().nw.clone(),
        )
    }

    fn next_generation(self) -> Self {
        debug_assert!(self.level >= 2, "must be level 2 or higher");

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

            Self::new(
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
            )
        }
    }
}

impl Node {
    fn manual_simulation(self) -> Self {
        let inode = self.inode_ref();
        debug_assert!(inode.level == 2, "manual simulation only for level 2");

        let mut all_bits = 0;
        for y in -2..2 {
            for x in -2..2 {
                // TODO: check if clone is necessary
                all_bits = (all_bits << 1) + self.get_bit(x, y);
            }
        }
        Self::new_inner(
            Self::one_gen(all_bits >> 5).into(),
            Self::one_gen(all_bits >> 4).into(),
            Self::one_gen(all_bits >> 1).into(),
            Self::one_gen(all_bits).into(),
        )
    }

    fn one_gen(mut bitmask: u16) -> Self {
        if bitmask == 0 {
            return Self::new_leaf(false);
        }

        let me = (bitmask >> 5) & 1;
        bitmask &= 0x757; // mask out bits we don't care about (?)
        let mut neighbor_count = 0;
        while bitmask != 0 {
            neighbor_count += 1;
            bitmask &= bitmask - 1; // clear least significant bit
        }
        if neighbor_count == 3 || (neighbor_count == 2 && me != 0) {
            Self::new_leaf(true)
        } else {
            Self::new_leaf(false)
        }
    }
}
