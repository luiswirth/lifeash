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

use crate::{node::Cell, universe::Universe};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
    EventPump,
};

const CELL_SIZE: u32 = 10;
const CELL_PADDING: u32 = 2;

pub struct Simulator {
    universe: Universe,
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl Simulator {
    pub fn new() -> Simulator {
        let mut universe = Universe::new();
        universe.initalize();

        // init sdl
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(env!("CARGO_PKG_NAME"), 1600, 1200)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Simulator {
            universe,
            canvas,
            event_pump,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.render();
            self.update();
        }
    }

    fn update(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => std::process::exit(0),
                _ => {}
            }
        }

        self.universe.evolve();
    }

    pub fn render(&mut self) {
        let canvas = &mut self.canvas;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        let center = {
            let size = canvas.viewport();
            Point::new(size.width() as i32 / 2, size.height() as i32 / 2)
        };

        let x_range = center.x() / CELL_SIZE as i32;
        let y_range = center.y() / CELL_SIZE as i32;

        for y in -y_range..y_range {
            for x in -x_range..x_range {
                let alive = match self.universe.get_cell((x as i64, y as i64)) {
                    Cell::Dead => false,
                    Cell::Alive => true,
                };

                if alive {
                    let center = center
                        + Point::new(
                            x as i32 * (CELL_SIZE + CELL_PADDING) as i32,
                            y as i32 * (CELL_SIZE + CELL_PADDING) as i32,
                        );
                    let rect = Rect::from_center(center, CELL_SIZE, CELL_SIZE);

                    canvas.fill_rect(rect).unwrap();
                }
            }
        }

        canvas.present();
        //std::thread::sleep(std::time::Duration::from_millis(10));
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
