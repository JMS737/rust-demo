mod collection_examples;

use collection_examples::pig_latin;
use collection_examples::vec_and_map;

fn main() {
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
}
