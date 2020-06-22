use sdl2::{
    rect::{Point, Rect},
    video::Window,
};

use std::convert::TryFrom;

use super::renderer::{CELL_PADDING, CELL_SIZE};
use la::Position;

pub const CAMERA_SPEED: f32 = 1.0;
pub const ZOOM_FACTOR: f32 = 1.1;

pub struct Camera {
    pub position: (f32, f32),
    pub zoom_level: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            zoom_level: 1.0,
            position: (0.0, 0.0),
        }
    }

    pub fn x_range(&self, window: &Window) -> std::ops::Range<i64> {
        let cell_size = (CELL_SIZE + CELL_PADDING) as f32 * self.zoom_level;
        let x_count = window.size().0 as f32 / cell_size;
        let min = self.position.0 as i64 - (x_count / 2.0).ceil() as i64 - 1;
        let max = self.position.0 as i64 + (x_count / 2.0).ceil() as i64 + 1;
        min..max
    }

    pub fn y_range(&self, window: &Window) -> std::ops::Range<i64> {
        let cell_size = (CELL_SIZE + CELL_PADDING) as f32 * self.zoom_level;
        let y_count = window.size().1 as f32 / cell_size;
        let min = self.position.1 as i64 - (y_count / 2.0).ceil() as i64 - 1;
        let max = self.position.1 as i64 + (y_count / 2.0).ceil() as i64 + 1;
        min..max
    }

    pub fn project(&self, window: &Window, pos: impl Into<Position>) -> Rect {
        let pos = pos.into();

        let mut point = Point::new(i32::try_from(pos.x).unwrap(), i32::try_from(pos.y).unwrap());

        // translate to camera pos
        point.x -= self.position.0 as i32;
        point.y -= self.position.1 as i32;

        // quadtree pos -> pixel pos
        point = Point::new(
            point.x as i32 * (CELL_SIZE + CELL_PADDING) as i32,
            point.y as i32 * (CELL_SIZE + CELL_PADDING) as i32,
        );

        // scale to zoom
        point = Point::new(
            (point.x as f32 * self.zoom_level) as i32,
            (point.y as f32 * self.zoom_level) as i32,
        );

        // translate to window center
        point += {
            let size = window.size();
            Point::new(size.0 as i32 / 2, size.1 as i32 / 2)
        };

        let size = (CELL_SIZE as f32 * self.zoom_level) as u32;
        Rect::from_center(point, size, size)
    }
}
