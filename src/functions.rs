use crate::model::Point;

#[allow(unused)]
pub fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    if a.is_left_of(&b) {
        return manhattan_distance(&a.increase_x(), &b) + 1;
    }

    if a.is_right_of(&b) {
        return manhattan_distance(&a.decrease_x(), &b) + 1;
    }

    if a.is_above_of(&b) {
        return manhattan_distance(&a.decrease_y(), &b) + 1;
    }

    if a.is_below_of(&b) {
        return manhattan_distance(&a.increase_y(), &b) + 1;
    }

    return 0;
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
}
