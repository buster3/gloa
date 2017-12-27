use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate shorter;



fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");
    let mut book_in = String::new();
    f.read_to_string(&mut book_in)
        .expect("something went wrong reading the file");

    println!("Words in book {}", shorter::book_shorter::number_words(&book_in));
    println!("Longest word in poem {}", shorter::book_shorter::longest_word(&book_in));

    /*
    let hist = shorter::book_shorter::compress(&book_in);
    for (i, x) in hist.iter().enumerate() {
        print!("{}, {}: ", i+1, x);
        for _ in 0..x/200 {
            print!("x");
        }
        println!("");
    }
    */

    let result = shorter::book_shorter::compress(&book_in);
    println!("Lines in result: {}", result.len());
    for (i, x) in result.iter().enumerate() {
        if *x != shorter::book_shorter::TARGET {
            print!("x");
        }
    }
}
