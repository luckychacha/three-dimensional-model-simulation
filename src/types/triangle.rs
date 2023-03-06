use cgmath::{InnerSpace, Point3};

use super::line_segment::LineSegment;

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

    pub fn intersection_with_line(&self, line: &LineSegment) -> Option<Point3<f32>> {
        const EPSILON: f32 = 0.000001;

        let v1 = self.vertices[0];
        let v2 = self.vertices[1];
        let v3 = self.vertices[2];
        let dir = line.end - line.start;
        let orig = line.start;
        let edge1 = v2 - v1;
        let edge2 = v3 - v1;
        let h = dir.cross(edge2);
        let a = edge1.dot(h);
        if a > -EPSILON && a < EPSILON {
            return None;
        }
        let f = 1.0 / a;
        let s = orig - v1;
        let u = f * s.dot(h);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let q = s.cross(edge1);
        let v = f * dir.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = f * edge2.dot(q);
        if t > EPSILON {
            return Some(orig + t * dir);
        }
        None
    }
}
