pub mod math;
pub mod rectangle;

#[cfg(test)]
mod tests {
    use crate::{math, rectangle::Rectangle};

    #[test]
    fn five_plus_five() {
        let result = math::add(5, 5);
        assert_eq!(result, 10);
    }

    #[test]
    fn five_plus_zero() {
        let result = math::add(5, 0);
        assert_eq!(result, 5);
    }

    #[test]
    fn zero_plus_zero() {
        let result = math::add(0, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn contains() {
        let big_rect = Rectangle::new(100, 200);
        let small_rect = Rectangle::new(40, 60);

        assert!(big_rect.contains(&small_rect));
    }

    #[test]
    fn does_not_contain() {
        let big_rect = Rectangle::new(100, 200);
        let small_rect = Rectangle::square(40);

        assert!(!small_rect.contains(&big_rect));
    }

    #[test]
    #[should_panic]
    fn divide_panic() {
        math::divide(10, 0);
    }
}
