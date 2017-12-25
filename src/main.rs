use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate shorter;



fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");
    let mut poem_in = String::new();
    f.read_to_string(&mut poem_in)
        .expect("something went wrong reading the file");

    println!("Words in poem {}", shorter::poem_shorter::number_words(&poem_in));
    println!("Longest word in poem {}", shorter::poem_shorter::longest_word(&poem_in));

    for x in shorter::poem_shorter::hist_word(&poem_in) {
        for _ in 1..x {
            print!("x");
        }
        println!("");
    }
}
