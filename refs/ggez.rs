pub fn run() -> Result<()> {
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
        .dimensions((width * cell_size) as f32, (height * cell_size) as f32)
        .resizable(true);

    let (mut ctx, mut event_loop) = ContextBuilder::new("life", "Luis Wirth")
        .window_setup(setup)
        .window_mode(mode)
        .build()
        .expect("ggez context could not be created");

    let mut app = App::new(&mut ctx, width, height, cell_size);

    event::run(&mut ctx, &mut event_loop, &mut app).wrap_err("ggez error")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

pub struct Universe {
    width: u32,
    height: u32,
    cell_size: u32,
    cells: Vec<Cell>,
}

impl Universe {
    #[inline(always)]
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for x_offset in &[-1, 0, 1] {
            for y_offset in &[-1, 0, 1] {
                if *x_offset == 0 && *y_offset == 0 {
                    continue;
                }
                let x = (column as i32 + x_offset).abs() % self.width as i32;
                let y = (row as i32 + y_offset).abs() % self.height as i32;
                let neighbor = self.cells[self.get_index(y as u32, x as u32)];
                match neighbor {
                    Cell::Dead => {}
                    Cell::Alive => count += 1,
                }
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
                    let x = col as f32;
                    let y = row as f32;
                    let r: u8 = (x + y) as u8;
                    let g: u8 = (x / y) as u8;
                    let b: u8 = (x * y) as u8;
                    graphics::Color::from((r, g, b))
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

        graphics::present(ctx)
    }
}
