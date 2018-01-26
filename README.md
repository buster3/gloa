# Gloa
A book shorter by Thomas Maierbacher <maierbacher@gmail.com>

## Challange
https://wunder.dog/the-shortest-edition/

## Install dependencies
The program is written in RUST. Install the RUST toolchain to build the program.
https://www.rust-lang.org/en-US/install.html

## Build and run
```bash
cargo run --release  <path_to_alastalon_salissa.txt>
```
The reduced version of the book is written to `out.txt`.

## Validator result
```
All words found:    [✓]
Lines max 80 chars: [✓]
21162 lines
```

## Version2
Only 46 characters in line 21162. This should be space minimal solution.

## Speed
```
time target/release/gloa ../alastalon_salissa.txt
Lines in result: 21162

real    0m0.122s
user    0m0.032s
sys     0m0.040s
```
