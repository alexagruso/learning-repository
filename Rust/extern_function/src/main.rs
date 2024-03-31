extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Abs(-3) = {}", abs(-3));
    }
}
