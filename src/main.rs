use cgmath::Point3;
use three_dimensional_model_simulation::{
    found_intersections, load_from_file, types::line_segment::LineSegment, write_to_file,
};

fn main() {
    let line = LineSegment {
        start: Point3 {
            x: 1.1,
            y: 1.0,
            z: 1.3,
        },
        end: Point3 {
            x: 9.0,
            y: 9.9,
            z: 9.0,
        },
    };
    let model = String::from("cube_6.obj");
    let obj = load_from_file(model).expect("load obj file failed.");
    if let Ok(intersection) = found_intersections(line, &obj) {
        write_to_file(intersection, &obj, &line, "output.obj")
            .expect("write into new obj file failed.");
    }
}
