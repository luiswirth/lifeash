#[derive(Debug, Clone)]
pub(super) enum Node {
    // always level 0
    Leaf(Cell),
    // Node::Inner can never have level 0
    Inner(Inode),
}

#[derive(Debug, Clone)]
pub(super) struct Inode {
    pub(super) level: usize,
    pub(super) population: usize,
    pub(super) nw: Box<Node>,
    pub(super) ne: Box<Node>,
    pub(super) sw: Box<Node>,
    pub(super) se: Box<Node>,
}

// reduce to bit
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum Cell {
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
    pub(super) fn new(nw: Node, ne: Node, sw: Node, se: Node) -> Self {
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
    pub(super) fn new_leaf(alive: bool) -> Self {
        Node::Leaf(Cell::new(alive))
    }

    #[inline(always)]
    pub(super) fn new_inner(nw: Node, ne: Node, sw: Node, se: Node) -> Self {
        Node::Inner(Inode::new(nw, ne, sw, se))
    }

    #[allow(unused)]
    #[inline(always)]
    pub(super) fn cell(self) -> Cell {
        if let Node::Leaf(cell) = self {
            cell
        } else {
            panic!("not a leaf")
        }
    }

    #[allow(unused)]
    #[inline(always)]
    pub(super) fn cell_ref(&self) -> &Cell {
        if let Node::Leaf(ref cell) = self {
            cell
        } else {
            panic!("not a leaf")
        }
    }

    #[inline(always)]
    pub(super) fn inode(self) -> Inode {
        if let Node::Inner(inode) = self {
            inode
        } else {
            panic!("not an inner")
        }
    }

    #[inline(always)]
    pub(super) fn inode_ref(&self) -> &Inode {
        if let Node::Inner(ref inode) = self {
            inode
        } else {
            panic!("not an inner")
        }
    }
}

impl Node {
    pub(super) fn new_empty_tree(level: usize) -> Self {
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
    pub(super) fn set_bit(self, x: isize, y: isize) -> Self {
        match self {
            Node::Leaf(_) => Self::new_leaf(true),
            Node::Inner(Inode {
                level,
                population: _,
                nw,
                ne,
                sw,
                se,
            }) => {
                let offset = 1 << (level - 2);
                match (x < 0, y < 0) {
                    (true, true) => {
                        Self::new_inner(nw.set_bit(x + offset, y + offset), *ne, *sw, *se)
                    }
                    (true, false) => {
                        Self::new_inner(*nw, *ne, sw.set_bit(x + offset, y - offset), *se)
                    }
                    (false, true) => {
                        Self::new_inner(*nw, ne.set_bit(x - offset, y + offset), *sw, *se)
                    }
                    (false, false) => {
                        Self::new_inner(*nw, *ne, *sw, se.set_bit(x - offset, y - offset))
                    }
                }
            }
        }
    }

    pub(super) fn get_bit(&self, x: isize, y: isize) -> u16 {
        match self {
            Node::Leaf(c) => *c as u16,
            Node::Inner(Inode {
                level,
                population: _,
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

    pub(super) fn expand_universe(self) -> Self {
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
    pub(super) fn centered_sub(&self) -> Self {
        Self::new(
            *self.nw.inode_ref().se.clone(),
            *self.ne.inode_ref().sw.clone(),
            *self.sw.inode_ref().ne.clone(),
            *self.se.inode_ref().nw.clone(),
        )
    }

    pub(super) fn centered_horizontal(west: &Self, east: &Self) -> Self {
        debug_assert!(west.level == east.level, "levels must be the same");

        Self::new(
            *west.ne.inode_ref().se.clone(),
            *east.nw.inode_ref().sw.clone(),
            *west.se.inode_ref().ne.clone(),
            *east.sw.inode_ref().nw.clone(),
        )
    }

    pub(super) fn centered_vertical(north: &Self, south: &Self) -> Self {
        debug_assert!(north.level == south.level, "levels must be the same");

        Self::new(
            *north.sw.inode_ref().se.clone(),
            *north.se.inode_ref().sw.clone(),
            *south.sw.inode_ref().ne.clone(),
            *south.ne.inode_ref().nw.clone(),
        )
    }

    pub(super) fn centered_subsub(&self) -> Self {
        Self::new(
            *self.nw.inode_ref().se.inode_ref().se.clone(),
            *self.ne.inode_ref().sw.inode_ref().sw.clone(),
            *self.sw.inode_ref().ne.inode_ref().ne.clone(),
            *self.se.inode_ref().nw.inode_ref().nw.clone(),
        )
    }

    pub(super) fn next_generation(self) -> Self {
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
    // Inode at level 2 contains 16 cells
    // these can be represented by a bitmap of u16

    // to update a cell be have to look at 9 cells (itself and the 8 directly adjecent ones)
    // so we still have to use a u16 bitmap.

    // the bottom three bits 0..=2 are the south neighbors
    // bits 4..=6 are the current row with 5 being the cell itself
    // 8..=10 are the north neighbors

    // 15 14 13 12
    // 11 10  9  8
    //  7  6  5  4
    //  3  2  1  0

    pub(super) fn manual_simulation(self) -> Self {
        let inode = self.inode_ref();
        debug_assert!(inode.level == 2, "manual simulation only for level 2");

        let mut all_bits: u16 = 0;
        for y in -2..2 {
            for x in -2..2 {
                all_bits = (all_bits << 1) + self.get_bit(x, y);
            }
        }
        Self::new_inner(
            Self::one_gen(all_bits >> 5), // nw
            Self::one_gen(all_bits >> 4), // ne
            Self::one_gen(all_bits >> 1), // sw
            Self::one_gen(all_bits),      // se
        )
    }

    pub(super) fn one_gen(mut bitmask: u16) -> Self {
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
