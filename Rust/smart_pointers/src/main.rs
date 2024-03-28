use std::rc::Rc;

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

fn main() {
    use List::*;

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Rc: {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("Rc: {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("Rc: {}", Rc::strong_count(&a));
    }

    println!("Rc: {}", Rc::strong_count(&a));
}
