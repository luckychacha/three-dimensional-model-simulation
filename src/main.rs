use cgmath::Point3;
use three_dimensional_model_simulation::{
    found_intersections, load_from_file, types::line_segment::LineSegment, write_to_file,
};

fn main() {
    let line = LineSegment {
        start: Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        end: Point3 {
            x: 2.0,
            y: 0.0,
            z: 2.0,
        },
    };
    let model = String::from("assets/cube_6.obj");
    let obj = load_from_file(model).expect("load obj file failed.");
    if let Ok(intersection) = found_intersections(line, &obj) {
        write_to_file(intersection, &obj, &line, "assets/output.obj")
            .expect("write into new obj file failed.");
    }
}
