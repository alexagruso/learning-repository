use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let value = String::from("Hello, World!");
        tx.send(value.clone()).unwrap();
        println!("Sent {}", value);
    });

    let receiver = thread::spawn(move || {
        if let Ok(message) = rx.recv() {
            println!("Received {message}");
        }
    });

    receiver.join().unwrap();

    let (tx1, rx) = mpsc::channel();

    thread::spawn(move || {
        let tx2 = tx1.clone();

        let messages = vec![
            String::from("One message"),
            String::from("Two message"),
            String::from("Yellow message"),
            String::from("Blue message"),
        ];

        for message in messages.iter() {
            thread::sleep(Duration::from_secs(1));
            tx2.send(message.clone()).unwrap();
            println!("Sent {message} from 1");
            tx2.send(message.clone()).unwrap();
            println!("Sent {message} from 2");
        }
    });

    for message in rx {
        println!("Received: {message}");
    }
}
