use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};

use cgmath::{InnerSpace, Point3, Vector3};
use glium::{index::NoIndices, uniform, Display, DrawParameters, Program, Surface};
use obj::Obj;
use types::{intersection::Intersection, line_segment::LineSegment, triangle::Triangle};
pub mod types;

pub fn found_intersections(
    line: LineSegment,
    model: &Obj,
) -> Result<Intersection, Box<dyn std::error::Error>> {
    let mut intersections = Vec::new();
    let triangles = generate_triangles(model)?;
    for triangle in triangles {
        if let Some(intersection) = triangle.intersection_with_line(&line) {
            // intersections.push(intersection);
            // let square = create_square_from_intersection(intersection, &triangle);
            intersections.push(intersection);
        }
    }
    println!("{intersections:?}");
    let intersection = Intersection {
        intersections,
        edge_length: 5_f32,
    };

    Ok(intersection)
}

pub fn load_from_file(model: String) -> Result<Obj, Box<dyn std::error::Error>> {
    let file = File::open(model)?;
    let reader = BufReader::new(file);
    let obj: Obj = obj::load_obj(reader)?;

    Ok(obj)
}

fn generate_triangles(obj: &Obj) -> Result<Vec<Triangle>, Box<dyn std::error::Error>> {
    println!("vertices {}", obj.vertices.len());
    let positions = obj
        .vertices
        .iter()
        .map(|item| Point3::new(item.position[0], item.position[1], item.position[2]))
        .collect::<Vec<Point3<f32>>>();

    let triangles = obj
        .indices
        .chunks_exact(3)
        .map(|chunk| {
            Triangle::new(
                positions[chunk[0] as usize],
                positions[chunk[1] as usize],
                positions[chunk[2] as usize],
            )
        })
        .collect::<Vec<Triangle>>();

    println!("positions: {}", positions.len());
    println!("triangles: {}", triangles.len());
    Ok(triangles)
}

pub fn write_to_file(
    intersection: Intersection,
    obj: &Obj,
    _line: &LineSegment,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    // Write vertices
    for vertex in &obj.vertices {
        writeln!(
            &mut writer,
            "v {} {} {} 0 0 255",
            vertex.position[0], vertex.position[1], vertex.position[2]
        )?;
    }

    // Write triangles
    obj.indices.chunks_exact(3).for_each(|chunk| {
        let v1 = chunk[0] as u32 + 1;
        let v2 = chunk[1] as u32 + 1;
        let v3 = chunk[2] as u32 + 1;
        let _ = writeln!(writer, "f {v1} {v2} {v3}");
    });

    let mut current_vertices_count = obj.vertices.len();

    for intersection in intersection.intersections {
        // Calculate the four vertices of the tetrahedron.
        let v1 = intersection + Vector3::new(5.0, 0.0, 0.0);
        let v2 = intersection + Vector3::new(0.0, 5.0, 0.0);
        let v3 = intersection + Vector3::new(0.0, 0.0, 5.0);
        let v4 = intersection - Vector3::new(5.0, 5.0, 5.0);

        // add 4 vertices
        writeln!(&mut writer, "v {} {} {} 255 0 0", v1[0], v1[1], v1[2])?;
        writeln!(&mut writer, "v {} {} {} 255 0 0", v2[0], v2[1], v2[2])?;
        writeln!(&mut writer, "v {} {} {} 255 0 0", v3[0], v3[1], v3[2])?;
        writeln!(&mut writer, "v {} {} {} 255 0 0", v4[0], v4[1], v4[2])?;

        // add 4 face

        ((current_vertices_count + 1)..=(current_vertices_count + 4))
            .collect::<Vec<_>>()
            .as_slice()
            .chunks_exact(4)
            .for_each(|item| {
                let _ = writeln!(writer, "f {} {} {}", item[0], item[1], item[2]);
                let _ = writeln!(writer, "f {} {} {}", item[0], item[1], item[3]);
                let _ = writeln!(writer, "f {} {} {}", item[1], item[2], item[3]);
                let _ = writeln!(writer, "f {} {} {}", item[0], item[2], item[3]);
            });

        current_vertices_count += 4;
    }

    Ok(())
}

pub fn create_square_from_intersection(
    intersection: Point3<f32>,
    triangle: &Triangle,
) -> [Point3<f32>; 4] {
    let normal = triangle_normal(
        triangle.vertices[0],
        triangle.vertices[1],
        triangle.vertices[2],
    );
    let side1 = triangle.vertices[1] - triangle.vertices[0];
    let side2 = triangle.vertices[2] - triangle.vertices[0];

    let p1 = intersection + (normal * 0.01); // move slightly along the normal
    let p2 = intersection - (normal * 0.01); // move slightly opposite the normal

    let mut p3 = intersection + (side1.normalize() * 0.05);
    let mut p4 = intersection + (side2.normalize() * 0.05);

    let dir = Vector3::new(1.0, 1.0, 1.0);
    let dist = 0.05 * (2.0_f32).sqrt();
    p3 += (dir * dist).cross(side1).normalize() * 0.05;
    p4 += (dir * dist).cross(side2).normalize() * 0.05;

    [p1, p3, p2, p4]
}

fn triangle_normal(p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) -> Vector3<f32> {
    (p2 - p1).cross(p3 - p1).normalize()
}

pub fn draw_line(display: &Display, program: &Program, line: &LineSegment) {
    let vertex_buffer = line.to_vertex_buffer(display);
    let indices = NoIndices(glium::index::PrimitiveType::LinesList);

    let draw_params = DrawParameters {
        line_width: Some(1.0),
        ..Default::default()
    };

    let uniforms = uniform! {};

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);

    target
        .draw(&vertex_buffer, indices, program, &uniforms, &draw_params)
        .unwrap();

    target.finish().unwrap();
}
