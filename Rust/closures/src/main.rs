#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            }
        }

        if red_count < blue_count {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

struct StringWrapper {
    string: String,
}

impl StringWrapper {
    fn new(value: &str) -> Self {
        Self {
            string: value.to_string(),
        }
    }

    fn count_from(&self, f: impl Fn(&String) -> i32) -> i32 {
        f(&self.string)
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let user_giveaway1 = store.giveaway(user_pref1);
    println!("User one got {:?}", user_giveaway1);

    let user_pref2 = None;
    let user_giveaway2 = store.giveaway(user_pref2);
    println!("User two got {:?}\n", user_giveaway2);

    let string1 = StringWrapper::new("Hello, Hello, HELLO!");

    let count_l_ignore_case = |string: &String| -> i32 {
        let mut result: i32 = 0;

        for char in string.to_lowercase().as_bytes() {
            if *char == b'l' {
                result += 1;
            }
        }

        result
    };

    println!("L's: {}", string1.count_from(count_l_ignore_case));
}
