use cgmath::Point3;

pub struct Intersection {
    pub intersections: Vec<Point3<f32>>,
    pub edge_length: f32,
}
