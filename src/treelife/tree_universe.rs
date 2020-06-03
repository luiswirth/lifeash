use crate::treelife::tree_node::Node;
use crate::universe::Universe;

pub struct TreeUniverse {
    generation: usize,
    root: Node,
}

impl Default for TreeUniverse {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeUniverse {
    pub fn new() -> Self {
        Self {
            generation: 0,
            root: Node::new_empty_tree(3),
        }
    }
}

impl Universe for TreeUniverse {
    fn get_bit(&self, x: isize, y: isize) -> u16 {
        self.root.get_bit(x, y)
    }

    fn set_bit(&mut self, x: isize, y: isize) {
        let mut copy = self.root.clone();
        loop {
            let max_coordinate: isize = 1 << (self.root.inode_ref().level - 1);
            if -max_coordinate <= x
                && x < max_coordinate
                && -max_coordinate <= y
                && y < max_coordinate
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
