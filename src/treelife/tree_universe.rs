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
    fn get_bit(&self, x: i32, y: i32) -> u16 {
        self.root.get_bit(x, y)
    }

    fn set_bit(&mut self, x: i32, y: i32, alive: bool) {
        // TODO: remove copy?
        let mut copy = self.root.clone();
        loop {
            // check coordiante bounds
            let max = 2i32.pow(copy.level() - 1);
            let bound = (-max)..(max);
            if bound.contains(&x) && bound.contains(&y) {
                break;
            }
            copy = copy.expand_universe();
        }

        self.root = copy.set_bit(x, y, alive);
    }

    fn run_step(&mut self) {
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
