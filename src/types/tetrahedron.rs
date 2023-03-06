use cgmath::{Point3, Vector3};

use super::intersection::Intersection;

pub struct Tetrahedron {
    pub top: Point3<f32>,
    pub bottoms: [Point3<f32>; 3],
}

impl From<&Intersection> for Tetrahedron {
    fn from(intersection: &Intersection) -> Self {
        Self {
            top: intersection.points + Vector3::new(intersection.edge_length, 0.0, 0.0),
            bottoms: [
                intersection.points + Vector3::new(0.0, intersection.edge_length, 0.0),
                intersection.points + Vector3::new(0.0, 0.0, intersection.edge_length),
                intersection.points
                    - Vector3::new(
                        intersection.edge_length,
                        intersection.edge_length,
                        intersection.edge_length,
                    ),
            ],
        }
    }
}
