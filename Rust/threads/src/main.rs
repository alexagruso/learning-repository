use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread iteration {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Main thread iteration {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let vector = vec![1, 2, 4];

    let handle = thread::spawn(move || {
        for _ in 1..5 {
            println!("Vector: {:?}", vector);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
}
