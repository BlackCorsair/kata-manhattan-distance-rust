use crate::functions::manhattan_distance;
use crate::model::Point;

mod functions;
mod model;

fn main() {
    let a = Point::new(100, 150);
    let b = Point::new(-40, 250);
    println!(
        "Hello, world! Manhattan distance between {:?} and {:?} is {:?}",
        &a, &b, manhattan_distance(&a, &b)
    );
}
