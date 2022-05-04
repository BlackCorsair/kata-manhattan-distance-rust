#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    #[allow(dead_code)]
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }

    pub fn is_left_of(&self, other: &Point) -> bool {
        return self.x < other.x;
    }

    pub fn is_right_of(&self, other: &Point) -> bool {
        return self.x > other.x;
    }

    pub fn is_above_of(&self, other: &Point) -> bool {
        return self.y > other.y;
    }

    pub fn is_below_of(&self, other: &Point) -> bool {
        return self.y < other.y;
    }

    pub fn increase_x(&self) -> Point {
        return Point::new(self.x + 1, self.y);
    }
    pub fn decrease_x(&self) -> Point {
        return Point::new(self.x - 1, self.y);
    }
    pub fn increase_y(&self) -> Point {
        return Point::new(self.x, self.y + 1);
    }
    pub fn decrease_y(&self) -> Point {
        return Point::new(self.x, self.y - 1);
    }
}

#[cfg(test)]
mod point_tests {
    use super::Point;

    #[test]
    fn is_left_of_works() {
        let a = Point::new(0, 0);
        let b = Point::new(1, 0);

        assert!(a.is_left_of(&b));
        assert!(!b.is_left_of(&a));
        assert!(!a.is_left_of(&a));
    }

    #[test]
    fn is_right_of_works() {
        let a = Point::new(0, 0);
        let b = Point::new(1, 0);

        assert!(!a.is_right_of(&b));
        assert!(b.is_right_of(&a));
        assert!(!a.is_right_of(&a));
    }

    #[test]
    fn is_above_of_works() {
        let a = Point::new(0, 0);
        let b = Point::new(0, 1);

        assert!(!a.is_above_of(&b));
        assert!(b.is_above_of(&a));
        assert!(!a.is_above_of(&a));
    }

    #[test]
    fn is_below_of_works() {
        let a = Point::new(0, 0);
        let b = Point::new(0, 1);

        assert!(a.is_below_of(&b));
        assert!(!b.is_below_of(&a));
        assert!(!a.is_below_of(&a));
    }

    #[test]
    fn increase_x_works() {
        let a = Point::new(0, 0);

        assert_eq!(Point::new(1, 0), a.increase_x());
    }

    #[test]
    fn decrease_x_works() {
        let a = Point::new(0, 0);

        assert_eq!(Point::new(-1, 0), a.decrease_x());
    }

    #[test]
    fn increase_y_works() {
        let a = Point::new(0, 0);

        assert_eq!(Point::new(0, 1), a.increase_y());
    }

    #[test]
    fn decrease_y_works() {
        let a = Point::new(0, 0);

        assert_eq!(Point::new(0, -1), a.decrease_y());
    }
}
