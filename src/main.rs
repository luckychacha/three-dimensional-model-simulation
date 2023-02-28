use cgmath::Point3;
use three_dimensional_model_simulation::{
    found_intersections, load_from_file, types::line_segment::LineSegment,
};

fn main() {
    let line = LineSegment {
        start: Point3 {
            x: -1000.0,
            y: -1000.0,
            z: -1000.0,
        },
        end: Point3 {
            x: -1000000.0,
            y: -1000000.0,
            z: -1000000.0,
        },
    };
    match found_intersections(line, String::from("body.obj")) {
        Ok(res) => {
            println!("res: {res:?}");
        }
        Err(e) => {
            eprintln!("error info:{e}");
        }
    }

    // let _ = load_from_file(String::from("body.obj"));
}
