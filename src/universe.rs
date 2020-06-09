use crate::core::{Level, Position};
use crate::node::Node;

pub struct Universe {
    generation: usize,
    root: Node,
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

impl Universe {
    pub fn new() -> Self {
        Self {
            generation: 0,
            root: Node::new_empty_tree(Level(3)),
        }
    }

    pub fn get_bit(&self, pos: impl Into<Position>) -> u16 {
        self.root.get_bit(pos)
    }

    // TODO: modernize
    pub fn set_bit(&mut self, pos: impl Into<Position>, alive: bool) {
        let pos = pos.into();
        // TODO: remove copy?
        let mut copy = self.root.clone();
        loop {
            // check coordiante bounds
            if pos.in_bounds(copy.level()) {
                break;
            }
            copy = copy.expand_universe();
        }

        self.root = copy.set_bit(pos, alive);
    }

    pub fn run_step(&mut self) {
        while self.root.level() < 3
            || self.root.inode_ref().nw.population()
                != self
                    .root
                    .inode_ref()
                    .nw
                    .inode_ref()
                    .se
                    .inode_ref()
                    .se
                    .population()
            || self.root.inode_ref().ne.population()
                != self
                    .root
                    .inode_ref()
                    .ne
                    .inode_ref()
                    .sw
                    .inode_ref()
                    .sw
                    .population()
            || self.root.inode_ref().sw.population()
                != self
                    .root
                    .inode_ref()
                    .sw
                    .inode_ref()
                    .ne
                    .inode_ref()
                    .ne
                    .population()
            || self.root.inode_ref().se.population()
                != self
                    .root
                    .inode_ref()
                    .se
                    .inode_ref()
                    .nw
                    .inode_ref()
                    .nw
                    .population()
        {
            self.root = self.root.clone().expand_universe();
        }

        self.root = self.root.clone().inode().next_generation().into();
        self.generation += 1;
    }
}
