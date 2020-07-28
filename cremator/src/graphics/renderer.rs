use std::time::Instant;

#[allow(unused)]
pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span, warn,
    warn_span,
};

use glium::{
    glutin::{
        self,
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
    },
    Display, Surface,
};
use imgui::{Context as ImguiContext, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer as ImguiRenderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

//use super::camera::{Camera, CAMERA_SPEED, ZOOM_FACTOR};
use la::{Cell, Universe};

use super::cell_renderer;

use cell_renderer::CellRenderer;

pub struct Renderer {
    imgui_context: ImguiContext,
    platform: WinitPlatform,
    imgui_renderer: ImguiRenderer,
    font_size: f32,
    cell_renderer: CellRenderer,
}

impl Renderer {
    pub fn init(display: &Display) -> Self {
        let mut imgui_context = ImguiContext::create();
        imgui_context.set_ini_filename(None);

        //if let Some(backend) = clipboard::init() {
        //imgui_context.set_clipboard_backend(Box::new(backend));
        //} else {
        //error!("Failed to initalize clipboard");
        //}

        let mut platform = WinitPlatform::init(&mut imgui_context);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui_context.io_mut(), &window, HiDpiMode::Rounded);
        }

        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui_context.fonts().add_font(&[
            FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: font_size,
                    ..FontConfig::default()
                }),
            },
            FontSource::TtfData {
                data: include_bytes!("../../../res/mplus-1p-regular.ttf"),
                size_pixels: font_size,
                config: Some(FontConfig {
                    rasterizer_multiply: 1.75,
                    glyph_ranges: FontGlyphRanges::japanese(),
                    ..FontConfig::default()
                }),
            },
        ]);

        imgui_context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        let imgui_renderer = ImguiRenderer::init(&mut imgui_context, display)
            .expect("Failed to create ImguiRenderer");

        let cell_renderer = CellRenderer::new(display);

        Self {
            imgui_context,
            platform,
            imgui_renderer,
            cell_renderer,
            font_size,
        }
    }

    pub fn handle_event(&mut self, event: Event<()>, display: &Display) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('w'),
                ..
            } => {} //self.camera.position.1 -= CAMERA_SPEED,
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('s'),
                ..
            } => {} //self.camera.position.1 += CAMERA_SPEED,
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('a'),
                ..
            } => {} //self.camera.position.0 -= CAMERA_SPEED,
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('d'),
                ..
            } => {} //self.camera.position.0 += CAMERA_SPEED,
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('q'),
                ..
            } => {} //self.camera.zoom_level /= ZOOM_FACTOR,
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('e'),
                ..
            } => {} //self.camera.zoom_level *= ZOOM_FACTOR,
            event => {
                self.platform.handle_event(
                    self.imgui_context.io_mut(),
                    display.gl_window().window(),
                    &event,
                );
            }
        }
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
