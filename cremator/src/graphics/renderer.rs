#[allow(unused)]
pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span, warn,
    warn_span,
};

use glium::{glutin::event::Event, Display, Surface};

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

        Self { cell_renderer }
    }

    pub fn handle_event(&mut self, event: Event<()>, display: &Display) {
        self.cell_renderer.handle_event(event, display)
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self, universe: &Universe, display: &Display) {
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        self.cell_renderer.render(universe, display, &mut frame);

        frame.finish().unwrap();
    }
}
