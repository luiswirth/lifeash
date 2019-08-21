use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cell_size: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn get_mesh(&self, ctx: &mut Context) -> GameResult<graphics::Mesh> {
        let mb = &mut graphics::MeshBuilder::new();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];

                let rect = graphics::Rect::new(
                    (col * self.cell_size) as f32,
                    (row * self.cell_size) as f32,
                    self.cell_size as f32,
                    self.cell_size as f32,
                );
                let color = if cell == Cell::Alive {
                    graphics::WHITE
                } else {
                    graphics::BLACK
                };
                mb.rectangle(graphics::DrawMode::fill(), rect, color);

            }
        }

        mb.build(ctx)
    }
}

impl Universe {
    pub fn new(width: u32, height: u32, cell_size: u32) -> Universe {
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cell_size,
            cells,
        }
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let mesh = self.get_mesh(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;

        graphics::present(ctx)?;
        Ok(())
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { ' ' } else { '█' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

// MAIN
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};

pub struct App {
    universe: Universe,
}

impl App {
    pub fn new(
        _ctx: &mut Context,
        universe_width: u32,
        universe_height: u32,
        cell_size: u32,
    ) -> App {
        let universe = Universe::new(universe_width, universe_height, cell_size);
        App { universe }
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.universe.tick();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        self.universe.render(ctx)?;

        graphics::present(ctx) // returns GameResult
    }
}

fn main() {
    let width: u32 = 256;
    let height: u32 = 256;
    let cell_size: u32 = 4;

    let setup = ggez::conf::WindowSetup {
        title: "Game of Life".to_owned(),
        samples: ggez::conf::NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let mode = ggez::conf::WindowMode::default()
        .dimensions(128.0 * 8.0, 128.0 * 8.0)
        .resizable(true);

    let (mut ctx, mut event_loop) = ContextBuilder::new("my_app", "lwirth")
        .window_setup(setup)
        .window_mode(mode)
        .build()
        .expect("ggez context could not be created");

    let mut app = App::new(&mut ctx, width, height, cell_size);

    match event::run(&mut ctx, &mut event_loop, &mut app) {
        Ok(_) => println!("exited cleanly"),
        Err(e) => println! {"error occured: {}", e},
    }
}
