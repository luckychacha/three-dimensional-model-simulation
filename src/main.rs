use cgmath::Point3;
use three_dimensional_model_simulation::{found_intersections, types::line_segment::LineSegment};

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
    match found_intersections(line, model) {
        Ok(res) => {
            println!("res: {res:?}");
        }
        Err(e) => {
            eprintln!("error info:{e}");
        }
    }

    // let _ = load_from_file(String::from("body.obj"));
}
