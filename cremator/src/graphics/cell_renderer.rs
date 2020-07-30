use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
    },
    Display, Frame, Program, Surface};

use la::{Cell, Universe};

use super::camera::{Camera, CAMERA_SPEED, ZOOM_FACTOR};

pub const CELL_SIZE: f32 = 0.02;
pub const CELL_PADDING: f32 = 0.005;

pub struct CellRenderer {
    program: Program,
    camera: Camera,
}

impl CellRenderer {
    pub fn new(display: &Display) -> Self {
        let vertex_shader = r#"
            #version 140

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        
        "#;

        let fragment_shader = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let program = Program::from_source(display, vertex_shader, fragment_shader, None).unwrap();

        let camera = Camera::new();

        CellRenderer { program, camera }
    }

    pub fn handle_event(&mut self, event: Event<()>, display: &Display) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('w'),
                ..
            } => self.camera.position.1 -= CAMERA_SPEED,
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('s'),
                ..
            } => self.camera.position.1 += CAMERA_SPEED,
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('a'),
                ..
            } => self.camera.position.0 -= CAMERA_SPEED,
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('d'),
                ..
            } => self.camera.position.0 += CAMERA_SPEED,
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('q'),
                ..
            } => self.camera.zoom_level /= ZOOM_FACTOR,
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter('e'),
                ..
            } => self.camera.zoom_level *= ZOOM_FACTOR,
            _ => {},
        }
    }

    pub fn render(&mut self, universe: &Universe, display: &Display, target: &mut Frame) {
        // calculate range in which we have to Universe::get_cell
        let x_range = self.camera.x_range();
        let y_range = self.camera.y_range();

        let mut vertices: Vec<Vertex> = Vec::new();

        for y in y_range {
            for x in x_range.clone() {
                let alive = match universe.get_cell((x, y)) {
                    Cell::Dead => false,
                    Cell::Alive => true,
                };

                if alive {
                    let mut new = self.camera.project((x, y));
                    vertices.append(&mut new);
                }
            }
        }

        println!("vertex array length: {}", vertices.len());

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();

        // TODO: use actual IndexBuffer
        //let indices: Vec<u8> = vec![0, 1, 2, 3, 4, 5];

        //let index_buffer = glium::index::IndexBuffer::new(
        //display,
        //glium::index::PrimitiveType::TrianglesList,
        //&indices,
        //)
        //.unwrap();

        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Vertex { position: [x, y] }
    }
}

glium::implement_vertex!(Vertex, position);
