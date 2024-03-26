use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    HalfDollar,
}

impl Distribution<Coin> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Coin {
        match rng.gen_range(0..=4) {
            0 => Coin::Penny,
            1 => Coin::Nickel,
            2 => Coin::Dime,
            3 => Coin::Quarter,
            _ => Coin::HalfDollar,
        }
    }
}

fn value(coin: &Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::HalfDollar => 50,
    }
}

fn main() {
    let mut coins = Vec::new();

    for _ in 0..10 {
        coins.push(rand::random::<Coin>());
    }

    let mut total_value = 0;

    for coin in coins.iter() {
        total_value += value(coin);
    }

    println!("{total_value}");
}
