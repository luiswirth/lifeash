use super::cell_renderer::{Vertex, CELL_PADDING, CELL_SIZE};
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

    pub fn x_range(&self) -> std::ops::Range<i64> {
        let cell_size = (CELL_SIZE + CELL_PADDING) as f32 * self.zoom_level;
        let x_count = 2.0 / cell_size;
        let min = self.position.0 as i64 - (x_count / 2.0).ceil() as i64 - 1;
        let max = self.position.0 as i64 + (x_count / 2.0).ceil() as i64 + 1;
        min..max
    }

    pub fn y_range(&self) -> std::ops::Range<i64> {
        let cell_size = (CELL_SIZE + CELL_PADDING) as f32 * self.zoom_level;
        let y_count = 2.0 / cell_size;
        let min = self.position.1 as i64 - (y_count / 2.0).ceil() as i64 - 1;
        let max = self.position.1 as i64 + (y_count / 2.0).ceil() as i64 + 1;
        min..max
    }

    pub fn project(&self, pos: impl Into<Position>) -> Vec<Vertex> {
        let pos = pos.into();

        // TODO: safe check conversion
        let mut point: (f32, f32) = (pos.x as f32, pos.y as f32);

        // translate to camera pos
        point.0 -= self.position.0;
        point.1 -= self.position.1;

        // quadtree pos -> pixel pos
        point = (
            point.0 * (CELL_SIZE + CELL_PADDING),
            point.1 * (CELL_SIZE + CELL_PADDING),
        );

        // scale to zoom
        point = (point.0 * self.zoom_level, point.1 * self.zoom_level);

        // translate to window center
        //point += {
        //let size = window.size();
        //Point::new(size.0 as i32 / 2, size.1 as i32 / 2)
        //};

        let half_size = CELL_SIZE * self.zoom_level / 2.0;

        let nw = Vertex::new(point.0 - half_size, point.1 + half_size);
        let sw = Vertex::new(point.0 - half_size, point.1 - half_size);
        let se = Vertex::new(point.0 + half_size, point.1 - half_size);
        let ne = Vertex::new(point.0 + half_size, point.1 + half_size);

        vec![nw, sw, ne, ne, sw, se]
    }
}
