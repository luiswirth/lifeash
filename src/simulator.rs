#[allow(unused)]
use color_eyre::{Help, Report, Result};
#[allow(unused)]
use eyre::{eyre, WrapErr};

use std::io::prelude::*;

use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};

use crate::treelife::tree_universe::TreeUniverse;
use crate::universe::Universe;

const CELL_SIZE: isize = 5;

pub fn start_simulator() -> Result<()> {
    let setup = ggez::conf::WindowSetup {
        title: "Game of Life".to_owned(),
        samples: ggez::conf::NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let mode = ggez::conf::WindowMode::default().resizable(true);

    let (mut ctx, mut event_loop) = ContextBuilder::new("life", "Luis Wirth")
        .window_setup(setup)
        .window_mode(mode)
        .build()
        .expect("ggez context could not be created");

    let mut app = Simulator::new();
    app.read_pattern()?;

    event::run(&mut ctx, &mut event_loop, &mut app).wrap_err("error occured")?;
    Ok(())
}

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
}

impl EventHandler for Simulator {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.universe.run_step();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let mesh = self.get_mesh(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;

        graphics::present(ctx)
    }
}

impl Simulator {
    fn get_mesh(&self, ctx: &mut Context) -> GameResult<graphics::Mesh> {
        //self.universe.get_bit()

        let size = graphics::window(ctx).get_inner_size().unwrap();
        let (width, height) = (size.width as isize, size.height as isize);

        let mb = &mut graphics::MeshBuilder::new();

        for y in 0..height {
            for x in 0..width {
                let rect = graphics::Rect::new(
                    (x * CELL_SIZE) as f32,
                    (y * CELL_SIZE) as f32,
                    CELL_SIZE as f32,
                    CELL_SIZE as f32,
                );

                let bit = self.universe.get_bit(x, y);

                let color = if bit == 0 {
                    graphics::BLACK
                } else {
                    graphics::WHITE
                };

                mb.rectangle(graphics::DrawMode::fill(), rect, color);
            }
        }

        //let x = col as f32;
        //let y = row as f32;
        //let r: u8 = (x + y) as u8;
        //let g: u8 = (x / y) as u8;
        //let b: u8 = (x * y) as u8;

        mb.build(ctx)
    }

    fn read_pattern(&mut self) -> Result<()> {
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
