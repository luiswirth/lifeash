use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
    EventPump,
};

use hl::{Cell, Universe};

const CELL_SIZE: u32 = 10;
const CELL_PADDING: u32 = 2;

const CAMERA_SPEED: f32 = 10.0;

use nalgebra::{Matrix2, Point2, Rotation2};
type Matrix2f = Matrix2<f32>;

pub struct Renderer {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    camera: Camera,
}

struct Camera {
    position: Point2<f32>,
    zoom_level: f32,
}

impl Renderer {
    pub fn new() -> Self {
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

        let camera = Camera::new();

        Self {
            canvas,
            event_pump,
            camera,
        }
    }

    pub fn update(&mut self) {
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
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => self.camera.position.y -= CAMERA_SPEED,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => self.camera.position.y += CAMERA_SPEED,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => self.camera.position.x -= CAMERA_SPEED,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => self.camera.position.x += CAMERA_SPEED,
                _ => {}
            }
        }
    }

    pub fn render(&mut self, universe: &Universe) {
        let canvas = &mut self.canvas;

        let mvp = Matrix2::<f32>::identity();

        //mvp *= mat3.scale      1, -1             # invert the Y axis
        //mvp *= mat3.translate  gfx_w/2, gfx_h/2  # translate to center of screen
        //mvp *= mat3.rotate     cam.rotation      # rotate camera
        //mvp *= mat3.scale      cam.scale         # convert from units to pixels, 1 unit = gfx_w/40 pixels
        //mvp *= mat3.translate  -cam.x, -cam.y    # make (cam.x, cam.y) the center of the screen

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        let center = {
            let size = canvas.viewport();
            Point::new(
                size.width() as i32 / 2 - self.camera.position.x as i32,
                size.height() as i32 / 2 - self.camera.position.y as i32,
            )
        };

        let x_range = center.x() / CELL_SIZE as i32;
        let y_range = center.y() / CELL_SIZE as i32;

        for y in -y_range..y_range {
            for x in -x_range..x_range {
                let alive = match universe.get_cell((x as i64, y as i64)) {
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
}

impl Camera {
    fn new() -> Self {
        Self {
            zoom_level: 1.0,
            position: Point2::new(0.0, 0.0),
        }
    }

    fn set_zoom(&mut self, level: f32) {
        self.zoom_level = level;
    }

    fn set_position(&mut self, position: Point2<f32>) {
        self.position = position;
    }
}
