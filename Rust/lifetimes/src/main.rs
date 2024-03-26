fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        y
    } else {
        x
    }
}

fn main() {
    let hello = "hello, ";
    let world = "world!";

    let longest = longest(hello, world);

    println!("{}", longest);
}
