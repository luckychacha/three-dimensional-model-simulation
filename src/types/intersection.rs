use cgmath::Point3;

#[derive(Debug)]
pub struct Intersections {
    pub intersections: Vec<Intersection>,
}

#[derive(Debug)]
pub struct Intersection {
    pub points: Point3<f32>,
    pub edge_length: f32,
}

impl Intersection {
    pub fn new(points: Point3<f32>, edge_length: f32) -> Self {
        Self {
            points,
            edge_length,
        }
    }
}
