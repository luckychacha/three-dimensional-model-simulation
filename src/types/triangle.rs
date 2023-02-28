use cgmath::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vertices: [Point3<f32>; 3],
}
