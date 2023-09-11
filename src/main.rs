mod collection_examples;
mod file_examples;
mod fibonacci;
// mod trait_examples;
mod lifetime_examples;
mod thread_examples;

use std::thread;
use std::time::Duration;

use collection_examples::pig_latin;
use collection_examples::vec_and_map;

fn main() {
    thread_examples::start_work(2);
    println!("Hello, world!");
    let mut v = vec![1, 2, 3];

    v.push(4);

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    let item3 = v.get(2);

    if let Some(item3) = item3 {
        println!("{item3}");
    }

    let sequence = [1, 1, 2, 3, 4, 4, 4, 5, 6, 7];
    println!("Median: {}", vec_and_map::median(&sequence));
    println!("Mode: {}", vec_and_map::mode(&sequence));

    let sentence = "This is a random bit of text";

    println!("Pig latin: {}", pig_latin::from(sentence));
    println!("Original: {}", sentence);

    println!("Reading contents of 'hello.txt':");
    println!("<------- START -------->");
    let data = file_examples::read_file_improved("hello.txt").expect("Could not read data.");
    println!("{data}");
    println!("<-------- END --------->");
    println!("<------- START -------->");
    let data = file_examples::read_to_string("hello.txt").expect("Could not read data.");
    println!("{data}");
    println!("<-------- END --------->");

    file_examples::write_file("world.txt", "Hello there!");

    let n: usize = 20; // Max value of 186 can be calcuated before integer overflow of u128 type.
    print!("The first {n} numbers in the fibonacci sequence are: ");
    for i in fibonacci::generate(n) {
        print!("{i}, ");
    }
    println!();

    // Significantly slows down after n > 40.
    print!("The {n} number in the fibonacci sequence is: ");
    println!("{}", fibonacci::generate_recursive(n as u32));

    vec_and_map::vector_iteration();

    let string1 = "Hello";
    let string2 = "world!";

    println!("The longest of '{string1}' and '{string2}' is '{}'", lifetime_examples::longest(string1, string2));

    thread_examples::move_data_to_thread();

    vec_and_map::public_sum(&vec![1, 2, 3, 4]);

    thread_examples::example_mpsc();

}
