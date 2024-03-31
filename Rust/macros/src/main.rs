#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp = Vec::new();

            $(
                temp.push($x);
            )*

            temp
        }
    };
}

fn main() {
    let value = my_vec![1, 2, 3];

    for entry in value {
        println!("{entry}");
    }
}
