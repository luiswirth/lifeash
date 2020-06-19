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

use hl::{Cell, Universe};

use crate::graphics::renderer::Renderer;

pub struct Simulator {
    universe: Universe,
    renderer: Renderer,
}

impl Simulator {
    pub fn new() -> Simulator {
        let mut universe = Universe::new();
        universe.initalize();

        let renderer = Renderer::new();

        Simulator { universe, renderer }
    }

    pub fn run(&mut self) {
        loop {
            self.render();
            self.update();
        }
    }

    fn update(&mut self) {
        self.universe.evolve();
        self.renderer.update();
    }

    pub fn render(&mut self) {
        self.renderer.render();
    }

    pub fn read_rls(&mut self, pattern: &str) {
        let (mut x, mut y) = (0i64, 0i64);
        let mut argument: u32 = 0;

        for line in pattern.lines() {
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
                    '!' => return,
                    _ if c.is_digit(10) => {
                        argument = 10 * argument + c.to_digit(10).unwrap();
                    }
                    _ => panic!("invalid char"),
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn read_rls_from_stdin(&mut self) -> Result<()> {
        let mut string = String::new();

        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let mut line = String::new();

        while handle.read_line(&mut line)? != 0 {
            string.push_str(&line);
            if line.contains('!') {
                break;
            }
        }

        self.read_rls(&string);

        Ok(())
    }
}
