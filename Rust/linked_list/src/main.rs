struct LinkedList<T> {
    head: Option<Node<T>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

enum ConsList<T> {
    Cons(T, Box<ConsList<T>>),
    Nil,
}

fn main() {
    let linked_list = LinkedList::<i32> {
        head: Some(Node {
            data: 5,
            next: Some(Box::new(Node {
                data: 3,
                next: None,
            })),
        }),
    };

    let cons_list = ConsList::Cons(3, Box::new(ConsList::Cons(5, Box::new(ConsList::Nil))));

    println!(
        "{:?} -> {:?}",
        linked_list.head.as_ref().unwrap().data,
        linked_list
            .head
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .data
    );

    println!(
        "{} -> {}",
        match cons_list {
            ConsList::Cons(value, _) => value,
            ConsList::Nil => -1,
        },
        match cons_list {
            ConsList::Cons(_, next) => match *next {
                ConsList::Cons(value, _) => value,
                ConsList::Nil => -1,
            },
            ConsList::Nil => -1,
        }
    )
}
