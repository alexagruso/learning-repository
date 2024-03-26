use library::{float, integer};

fn main() {
    println!("3 + 7 = {}", integer::operations::add(3, 7));
    println!("3 * 7 = {}", integer::operations::multiply(3, 7));

    println!("3.8 + 1.3 = {}", float::operations::add(3.8, 1.3));
    println!("3.8 * 1.3 = {}", float::operations::multiply(3.8, 1.3));
}
