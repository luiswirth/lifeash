#[allow(unused)]
use color_eyre::{Help, Report, Result};
#[allow(unused)]
use eyre::{eyre, WrapErr};

#[allow(unused)]
pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span, warn,
    warn_span,
};

use std::io::prelude::*;

use crate::treelife::tree_universe::TreeUniverse;
use crate::universe::Universe;

pub struct Simulator {
    universe: Box<dyn Universe>,
}

impl Simulator {
    pub fn new() -> Simulator {
        //TODO: let user choose universe type
        Simulator {
            universe: Box::new(TreeUniverse::new()),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.update();
        }
    }

    fn update(&mut self) {
        self.universe.run_step();
    }

    pub fn read_pattern(&mut self) -> Result<()> {
        let mut line = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();

        let (mut x, mut y) = (0, 0);
        let mut argument = 0;

        while handle.read_line(&mut line)? != 0 {
            if line.starts_with('x') || line.starts_with('#') {
                continue;
            }
            let line = line.trim();
            for c in line.chars() {
                let parameter = if argument == 0 { 1 } else { argument };

                match c {
                    'b' => {
                        x += parameter;
                        argument = 0;
                    }
                    'o' => {
                        for _ in 0..parameter {
                            x += 1;
                            self.universe.set_bit(x, y)
                        }
                        argument = 0
                    }
                    '$' => {
                        y += parameter;
                        x = 0;
                        argument = 0;
                    }
                    _ if c.is_digit(10) => {
                        argument = 10 * argument + c.to_digit(10).unwrap() as isize
                    }
                    '!' => return Ok(()),
                    _ => panic!("invalid char"),
                }
            }
        }

        Ok(())
    }
}
