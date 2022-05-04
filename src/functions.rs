use crate::model::Point;

fn calculate_x_distance(a: &Point, b: &Point) -> i64 {
    let mut distance: i64 = 0;
    let mut pointer = *a;
    loop {
        match [pointer.is_left_of(&b), pointer.is_right_of(&b)] {
            [true, false] => { pointer = pointer.increase_x(); distance = distance + 1; }
            [false, true] => { pointer = pointer.decrease_x(); distance = distance + 1; }
            [false, false] => return distance,
            _ => panic!("NOT POSSIBLE")
        }
    }
}

fn calculate_y_distance(a: &Point, b: &Point) -> i64 {
    let mut distance: i64 = 0;
    let mut pointer = *a;
    loop {
        match [pointer.is_below_of(&b), pointer.is_above_of(&b)] {
            [true, false] => { pointer = pointer.increase_y(); distance = distance + 1; }
            [false, true] => { pointer = pointer.decrease_y(); distance = distance + 1; }
            [false, false] => return distance,
            _ => panic!("NOT POSSIBLE")
        }
    }
}

#[allow(unused)]
pub fn manhattan_distance(a: &Point, b: &Point) -> i64 {
    return calculate_y_distance(a, b) + calculate_x_distance(a, b)
}

#[cfg(test)]
mod manhattan_distance_tests {
    use super::manhattan_distance;
    use crate::model::Point;

    #[test]
    fn distance_is_zero() {
        let a = Point::new(0, 0);

        assert_eq!(0, manhattan_distance(&a, &a));
    }

    #[test]
    fn horizontal_distance() {
        let cardinal = Point::new(10, 10);
        let a = Point::new(11, 10);
        let b = Point::new(9, 10);

        assert_eq!(1, manhattan_distance(&cardinal, &a));
        assert_eq!(1, manhattan_distance(&cardinal, &b));
    }

    #[test]
    fn vertical_distance() {
        let cardinal = Point::new(10, 10);
        let a = Point::new(10, 11);
        let b = Point::new(10, 9);

        assert_eq!(1, manhattan_distance(&cardinal, &a));
        assert_eq!(1, manhattan_distance(&cardinal, &b));
    }

    #[test]
    fn diagonal_distance() {
        let cardinal = Point::new(10, 10);
        let a = Point::new(11, 11);
        let b = Point::new(9, 9);

        assert_eq!(2, manhattan_distance(&cardinal, &a));
        assert_eq!(2, manhattan_distance(&cardinal, &b));
    }
    #[test]
    fn distance_with_negative_numbers() {
        let cardinal = Point::new(0, 0);
        let a = Point::new(-10, -10);

        assert_eq!(20, manhattan_distance(&cardinal, &a));
        assert_eq!(20, manhattan_distance(&a, &cardinal));
    }

    #[test]
    fn giant_numbers() {
        let cardinal = Point::new(i32::MAX, i32::MAX);
        let a = Point::new(i32::MIN, i32::MIN);

        assert_eq!(8589934590, manhattan_distance(&cardinal, &a));
    }
}
