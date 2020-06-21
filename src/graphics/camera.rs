use sdl2::rect::{Point, Rect};

const CAMERA_SPEED: f32 = 10.0;
const ZOOM_FACTOR: f32 = 1.1;

const CELL_SIZE: u32 = 10;
const CELL_PADDING: u32 = 2;

pub struct Camera {
    position: (f32, f32),
    zoom_level: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            zoom_level: 1.0,
            position: (0.0, 0.0),
        }
    }

    pub fn set_zoom(&mut self, level: f32) {
        self.zoom_level = level;
    }

    pub fn set_position(&mut self, position: (f32, f32)) {
        self.position = position;
    }

    pub fn project(&self, x: i32, y: i32) -> Rect {
        let center = Point::new(
            x as i32 * (CELL_SIZE + CELL_PADDING) as i32,
            y as i32 * (CELL_SIZE + CELL_PADDING) as i32,
        );
        let mut rect = Rect::from_center(center, CELL_SIZE, CELL_SIZE);

        rect.set_x((x as f32 - self.position.0 * self.zoom_level) as i32);
        rect.set_y((y as f32 - self.position.1 * self.zoom_level) as i32);
        rect.set_x((x as f32 * self.zoom_level) as i32);
        rect.set_y((y as f32 * self.zoom_level) as i32);
        rect
    }
}
