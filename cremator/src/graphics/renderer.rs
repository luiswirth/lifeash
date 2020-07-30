use std::time::Instant;

#[allow(unused)]
pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span, warn,
    warn_span,
};

use glium::{
    Display, Surface,
    glutin::{
        event::{Event}
    }
};

//use super::camera::{Camera, CAMERA_SPEED, ZOOM_FACTOR};
use la::Universe;

use super::cell_renderer;

use cell_renderer::CellRenderer;

pub struct Renderer {
    cell_renderer: CellRenderer,
}

impl Renderer {
    pub fn init(display: &Display) -> Self {
        let cell_renderer = CellRenderer::new(display);

        Self {
            cell_renderer,
        }
    }

    pub fn handle_event(&mut self, event: Event<()>, display: &Display) {
        self.cell_renderer.handle_event(event, display)
    }

    pub fn update(&mut self) {
        // actually NewEvents
        //self.last_frame = self.imgui_context.io_mut().update_delta_time(last_frame)
        let mut last_frame = Instant::now();
    }

    pub fn render(&mut self, universe: &Universe, display: &Display) {
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        self.cell_renderer.render(universe, display, &mut frame);

        frame.finish().unwrap();
    }
}
