use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            position: [x, y, z],
        }
    }
}

// 要解决此问题，您需要为 Vertex 类型手动实现 glium::Vertex trait，或者使用 impl_vertex! 宏。
// impl_vertex! 宏可以帮助您自动实现 glium::Vertex trait，前提是您的 Vertex 类型的字段名称与 glium::Vertex trait 的默认字段名称匹配。
implement_vertex!(Vertex, position);
