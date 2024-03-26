use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Team 1"), 50);
    scores.insert(String::from("Team 2"), 25);

    for (key, value) in &scores {
        println!("{key} has {value} points");
    }

    scores.insert(String::from("Team 2"), 60);
    scores.entry(String::from("Team 3")).or_insert(45);

    for (key, value) in &scores {
        println!("{key} has {value} points");
    }
}
