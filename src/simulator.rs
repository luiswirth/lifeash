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

use crate::node::Cell;
use crate::universe::Universe;

pub struct Simulator {
    universe: Universe,
}

impl Simulator {
    pub fn new() -> Simulator {
        let universe = Universe::new();
        universe.initalize();

        Simulator { universe }
    }

    pub fn run(&mut self) {
        loop {
            self.render();
            self.update();
        }
    }

    fn update(&mut self) {
        self.universe.evolve();
    }

    pub fn render(&self) {
        for y in -8..8 {
            for x in -8..8 {
                let alive = match self.universe.get_cell((x, y)) {
                    Cell::Dead => false,
                    Cell::Alive => true,
                };
                if alive {
                    print!("o");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!();

        //std::thread::sleep(std::time::Duration::from_millis(1000));
        let mut string = String::new();
        std::io::stdin().read_line(&mut string).unwrap();
    }

    pub fn read_pattern(&mut self) -> Result<()> {
        let mut line = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();

        let (mut x, mut y) = (0i64, 0i64);
        let mut argument: u32 = 0;

        while handle.read_line(&mut line)? != 0 {
            if line.starts_with('x') || line.starts_with('#') {
                continue;
            }
            let line = line.trim();
            for c in line.chars() {
                let parameter: u32 = if argument == 0 { 1 } else { argument };

                match c {
                    'b' => {
                        x += parameter as i64;
                        argument = 0;
                    }
                    'o' => {
                        for _ in 0..parameter {
                            self.universe.set_cell((x, y), Cell::Alive);
                            x += 1;
                        }
                        argument = 0
                    }
                    '$' => {
                        y += parameter as i64;
                        x = 0;
                        argument = 0;
                    }
                    '!' => return Ok(()),
                    _ if c.is_digit(10) => {
                        argument = 10 * argument + c.to_digit(10).unwrap();
                    }
                    _ => panic!("invalid char"),
                }
            }
        }

        Ok(())
    }
}
