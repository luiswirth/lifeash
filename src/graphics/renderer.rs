use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump,
};

use hl::{Cell, Universe};

use super::camera::{Camera, CAMERA_SPEED, ZOOM_FACTOR};

pub const CELL_SIZE: u32 = 10;
pub const CELL_PADDING: u32 = 2;

pub struct Renderer {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    camera: Camera,
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
                } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => self.camera.position.1 -= CAMERA_SPEED,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => self.camera.position.1 += CAMERA_SPEED,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => self.camera.position.0 -= CAMERA_SPEED,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => self.camera.position.0 += CAMERA_SPEED,
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => self.camera.zoom_level *= ZOOM_FACTOR,
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => self.camera.zoom_level /= ZOOM_FACTOR,
                _ => {}
            }
        }
    }

    pub fn render(&mut self, universe: &Universe) {
        let canvas = &mut self.canvas;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::GREEN);

        // calculate range in which we have to Universe::get_cell
        let x_range = self.camera.x_range(canvas.window());
        let y_range = self.camera.y_range(canvas.window());

        for y in y_range {
            for x in x_range.clone() {
                let alive = match universe.get_cell((x, y)) {
                    Cell::Dead => false,
                    Cell::Alive => true,
                };

                if alive {
                    let rect = self.camera.project(canvas.window(), (x, y));
                    canvas.fill_rect(rect).unwrap();
                }
            }
        }

        canvas.present();
    }
}
