use std::io::{self};

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    fn square(side_length: i32) -> Self {
        Self {
            width: side_length,
            height: side_length,
        }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self) -> i32 {
        2 * self.width + 2 * self.height
    }

    fn contains(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }
}

fn main() {
    let mut width = String::new();
    let mut height = String::new();

    println!("Enter the width: ");

    io::stdin()
        .read_line(&mut width)
        .expect("Failed to read line!");

    println!("Enter the height: ");

    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line!");

    let width: i32 = width.trim().parse().expect("Enter an integer!");
    let height: i32 = height.trim().parse().expect("Enter an integer!");

    let rectangle = Rectangle::new(width, height);

    println!(
        "The area of {:?} is {} and its perimeter is {}",
        rectangle,
        rectangle.area(),
        rectangle.perimeter()
    );

    let other_rectangle = Rectangle::square(50);

    println!(
        "It is {} that\n{:?}\ncan contain\n{:?}",
        rectangle.contains(&other_rectangle),
        rectangle,
        other_rectangle
    );
}
