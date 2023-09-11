mod channels;

use std::{thread::{self, JoinHandle}, time::Duration};

// Re-expose a function from a sub-library so it can be accessed via `thread_examples::example_mpsc`
// rather than thread_examples::channels::example_mpsc`
pub use channels::example_mpsc;

pub fn move_data_to_thread() {
    let list = vec![1, 2, 3];

    println!("before closure: {:?}", list);

    // Must use the `move` keywork here to transfer ownership of the `list` variable to the new thread.
    // The new thread may outlive the calling thread so lifetimes cannot be guaranteed if a reference is used.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

pub fn start_work(n_threads: usize) {
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for i in 1..=n_threads {
        println!("Spawning thread number {}", i);
        handles.push(thread::spawn(do_work));
    }

    for handle in handles {
        handle.join().expect("Problem with thread");
    }
}

fn do_work() {
    for i in 1..=10 {
        println!("I'm running in a thread: {i}");
        thread::sleep(Duration::from_millis(100));
    }
}