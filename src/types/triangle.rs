use cgmath::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vertices: [Point3<f32>; 3],
}

impl Triangle {
    pub fn new(v1: Point3<f32>, v2: Point3<f32>, v3: Point3<f32>) -> Self {
        Self {
            vertices: [v1, v2, v3],
        }
    }
}
