use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate gloa;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} inputfile", args[0]);
    }
    let filename = &args[1];
    let mut f = File::open(filename).expect("File not found");
    let mut book_in = String::new();
    f.read_to_string(&mut book_in)
        .expect("something went wrong reading the file");

    //println!("Words in book {}", shorter::book_info::number_words(&book_in));
    //println!("Longest word in poem {}", shorter::book_info::longest_word(&book_in));
    //println!("Minimum number of lines {}", shorter::book_info::minimum_lines_possible(&book_in));

    let result = gloa::book_shorter::compress(&book_in);
    println!("Lines in result: {}", result.chars().count() as f32 / 81.);

    let mut out = File::create("out.txt").expect("Error creating output file");
    out.write_all(result.as_bytes()).expect("Error writing output");
    //print!("{}", result);
}
