fn evaluate(long_tuple: (i32, i32, i32, i32, i32)) {
    match long_tuple {
        (2, ..) => println!("First element is two!"),
        (.., 16) => println!("Last element is sixteen!"),
        (first, .., last) => {
            println!("First element: {first}, is not two.");
            println!("Last element: {last}, is not sixteen.")
        }
    };
}

fn main() {
    let mut values = vec![5, 193, 1351, 1395193];

    while let Some(top) = values.pop() {
        println!("{}", top)
    }

    let tuple1 = (1, 2, 4, 8, 16);
    let tuple2 = (2, 2, 4, 8, 16);
    let tuple3 = (1, 2, 4, 8, 15);

    evaluate(tuple1);
    evaluate(tuple2);
    evaluate(tuple3);

    struct IntegerWrapper {
        value: i32,
    }

    let value = IntegerWrapper { value: 5 };

    match value {
        IntegerWrapper {
            value: value @ 0..=10,
        } => println!("Has value {value}"),
        _ => (),
    };
}
