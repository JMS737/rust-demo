use std::thread;
use std::sync::mpsc;
use std::time::Duration;

/// Executes and example of a Multiple Producer, Single Consumer message channel.
pub fn example_mpsc() {
    // Note: the type String is inferred by the compiler when we pass Strings into the send method.
    let (tx, rx) = mpsc::channel();

    // Create a copy of the transmitter before it's moved to the thread, so we can have another thread posting messages
    // to the same channel.
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // unwrap to ensure success, else panic.
            tx.send(val).unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    });

    // Iterate over the receiver until the transmitter is dropped and all items have been received.
    for received in rx {
        println!("Received: {}", received);
    }
}