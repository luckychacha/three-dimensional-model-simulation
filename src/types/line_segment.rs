use cgmath::Point3;
use glium::{Display, VertexBuffer};

use super::vertex::Vertex;

#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    pub start: Point3<f32>,
    pub end: Point3<f32>,
}

impl LineSegment {
    pub fn to_vertex_buffer(&self, display: &Display) -> VertexBuffer<Vertex> {
        let vertices = [
            Vertex::new(self.start.x, self.start.y, self.start.z),
            Vertex::new(self.end.x, self.end.y, self.end.z),
        ];
        VertexBuffer::new(display, &vertices).unwrap()
    }
}
