#[derive(Debug)]
pub struct Rectangle {
    width: i32,
    height: i32,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn square(side_length: i32) -> Self {
        Self {
            width: side_length,
            height: side_length,
        }
    }

    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> i32 {
        2 * self.width + 2 * self.height
    }

    pub fn contains(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }
}
