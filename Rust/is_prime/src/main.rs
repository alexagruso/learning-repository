fn is_prime(value: i32) -> bool {
    if value == 1 {
        return false;
    }

    for i in 2..value {
        if value % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let mut prime_options: Vec<Option<i32>> = Vec::new();

    for i in 1..=100 {
        if is_prime(i) {
            prime_options.push(Some(i));
        } else {
            prime_options.push(None);
        }
    }

    for &entry in prime_options.iter() {
        if let Some(value) = entry {
            println!("{value} is prime!");
        }
    }
}
