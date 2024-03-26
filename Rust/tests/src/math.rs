#[allow(dead_code)]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[allow(dead_code)]
pub fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

#[allow(dead_code)]
pub fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

#[allow(dead_code)]
pub fn divide(x: i32, y: i32) -> i32 {
    if y == 0 {
        panic!("Denominator cannot be zero!");
    }

    x / y
}
