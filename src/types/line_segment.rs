use cgmath::Point3;

#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    pub start: Point3<f32>,
    pub end: Point3<f32>,
}
