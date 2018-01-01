use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate gloa;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <inputfile>", args[0]);
        return;
    }
    let filename = &args[1];
    let mut f = File::open(filename).expect("File not found");
    let mut book_in = String::new();
    f.read_to_string(&mut book_in)
        .expect("something went wrong reading the file");

    //println!("Words in book {}", shorter::book_info::number_words(&book_in));
    //println!("Longest word in poem {}", shorter::book_info::longest_word(&book_in));
    //println!("Minimum number of lines {}", shorter::book_info::minimum_lines_possible(&book_in));

    let mut out = File::create("out.txt").expect("Error creating output file");

    let mut line_cnt = 0;
    let ln_iterator = gloa::book_shorter::LineIterator::new(&book_in);
    for s in ln_iterator {
        out.write_all(s.as_bytes()).expect("Error writing output");
        line_cnt += 1;
    }
    println!("Lines in result: {}", line_cnt);
}
