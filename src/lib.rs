use std::{fs::File, io::BufReader};

use cgmath::{InnerSpace, Point3};
use obj::Obj;
use types::{line_segment::LineSegment, triangle::Triangle};
pub mod types;

fn intersect_triangle(line: &LineSegment, triangle: &Triangle) -> Option<Point3<f32>> {
    const EPSILON: f32 = 0.000001;

    let v1 = triangle.vertices[0];
    let v2 = triangle.vertices[1];
    let v3 = triangle.vertices[2];
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
    if u < 0.0 || u > 1.0 {
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

pub fn found_intersections(
    line: LineSegment,
    model: String,
) -> Result<Vec<Point3<f32>>, Box<dyn std::error::Error>> {
    let mut intersections = Vec::new();
    let triangles = load_from_file(model)?;

    for triangle in triangles {
        if let Some(intersection) = intersect_triangle(&line, &triangle) {
            intersections.push(intersection);
        }
    }

    println!("intersections:{intersections:?}");
    Ok(intersections)
}

pub fn load_from_file(model: String) -> Result<Vec<Triangle>, Box<dyn std::error::Error>> {
    let file = File::open(&model)?;
    let reader = BufReader::new(file);
    let obj: Obj = obj::load_obj(reader).unwrap();

    // 获取顶点和面数据
    // let positions = obj
    //     .positions
    //     .iter()
    //     .map(|p| Point3::new(p.x as f32, p.y as f32, p.z as f32))
    //     .collect::<Vec<Point3<f32>>>();
    let positions = obj
        .vertices
        .iter()
        .map(|item| Point3::new(item.position[0], item.position[1], item.position[2]))
        .collect::<Vec<Point3<f32>>>();

    let triangles = obj
        .indices
        .chunks_exact(3)
        .map(|chunk| Triangle {
            vertices: [
                positions[chunk[0] as usize],
                positions[chunk[1] as usize],
                positions[chunk[2] as usize],
            ],
        })
        .collect::<Vec<Triangle>>();

    println!("positions: {}", positions.len());
    println!("triangles: {}", triangles.len());
    Ok(triangles)
}
