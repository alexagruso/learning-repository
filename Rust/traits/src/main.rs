pub trait ToString {
    fn to_string(&self) -> String;
}

struct Point {
    x: i32,
    y: i32,
}

struct VecI32 {
    v: Vec<i32>,
}

impl ToString for Point {
    fn to_string(&self) -> String {
        String::from(format!("({}, {})", self.x, self.y))
    }
}

impl ToString for VecI32 {
    fn to_string(&self) -> String {
        let mut vector_string = String::from("( ");

        for value in &self.v {
            vector_string.push_str(&format!("{} ", value));
        }

        vector_string.push_str(")");
        vector_string
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    let v = VecI32 {
        v: vec![1, 5, 6, 1, 1515],
    };

    println!("{}", p.to_string());
    println!("{}", v.to_string());
}
