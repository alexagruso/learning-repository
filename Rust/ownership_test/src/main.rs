fn clone_longest(x: &str, y: &str) -> String {
    if x.len() >= y.len() {
        x.to_string().clone()
    } else {
        y.to_string().clone()
    }
}

fn main() {
    let x = String::from("Hello,");
    let y = String::from("Hello, World");

    let longest = clone_longest(&x, &y);

    println!("The longest between {} and {} is {}", x, y, longest);
}
