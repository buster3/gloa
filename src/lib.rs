pub mod book_info;

pub mod book_shorter {

    pub const TARGET: isize = 81;

    struct Word<'a> {
        w: &'a str
    }

    fn load_data(book_in: &str) -> Vec<Vec<Word>> {
        let mut vec: Vec<Vec<Word>> = vec![Vec::new()];
        for x in book_in.split_whitespace() {
            let cnt = x.chars().count();
            while vec.len() < cnt {
                vec.push(Vec::new())
            }
            vec[cnt - 1].push(Word{w: x});
        }
        vec
    }

    fn word_len(i: usize) -> isize {
        i as isize + 2
    }

    fn get_idx(i: isize) -> usize {
        assert!(i >= 2);
        i as usize - 2
    }

    pub const MAX_COMBINATIONS: usize = 3;


    fn calculate_fill_border(sorted: &Vec<Vec<Word>>, i: usize) -> isize {
        let mut border = word_len(i);
        let mut last_left = sorted[i].len() - 1;
        let mut last_idx = i;

        for _ in 1..MAX_COMBINATIONS {
            let mut idx = last_idx;
            if last_left == 0 {
                while {
                    idx -= 1;
                    sorted[idx].len() == 0 && idx > 0
                } {}
                if sorted[idx].len() == 0 {
                    break;
                }
                last_left = sorted[idx].len() - 1;
            } else {
                last_left -= 1;
            }
            last_idx = idx;
            border += word_len(idx);
        }
        border
    }


    fn try_combination(combination: &[usize; MAX_COMBINATIONS], sorted: &mut Vec<Vec<Word>>, res: &mut String, current_fill_level : &mut isize) -> bool {
        if sorted[combination[0]].len() == 0 {
            return false
        }
        let mut last_left = sorted[combination[0]].len() - 1;
        let mut last_idx = combination[0];

        let combination_possible = combination[1..MAX_COMBINATIONS].iter().all(|&idx| -> bool {
            if idx == last_idx {
                if last_left == 0 {
                    return false
                } else {
                    last_left -= 1;
                }
            } else {
                if sorted[idx].len() == 0 {
                    return false
                } else {
                    last_left = sorted[idx].len() - 1;
                }
            }
            last_idx = idx;
            true
        });

        if combination_possible {
            // found a pair to fill the line
            *current_fill_level += combination.iter().map(|&x| {
                res.push_str(sorted[x].pop().unwrap().w);
                res.push(' ');
                word_len(x)
            }).sum();
            true
        } else {
            false
        }
    }

    pub fn compress(book_in: &str) -> String {
        let mut sorted = load_data(book_in);
        let mut res: String = String::new();

        let mut current_fill_level = 0;
        for i in (0..sorted.len()).rev() {
            while sorted[i].len() > 0 {

                let free = TARGET - current_fill_level;

                // calculate aprox border for speedup
                let aprox_border = word_len(i) * MAX_COMBINATIONS as isize;
                let exact_border = calculate_fill_border(&sorted, i);
                //println!("free {} aprox_border {} exact_border {}", free, aprox_border, exact_border);
                if free > aprox_border || free > exact_border {
                    // fill up
                    current_fill_level += word_len(i);
                    res.push_str(sorted[i].pop().unwrap().w);
                    res.push(' ');
                    //println!("filled up {} current_fill_level {}", word_len(i), current_fill_level);
                } else {
                    // Try to fill up with exactly three more words.
                    for k in 0..2 {
                        let first_idx = i - k;
                        let mut second_idx = first_idx;
                        loop {
                            let word_len_third = free - word_len(first_idx) - word_len(second_idx);
                            if word_len_third >= 2 {
                                let combination = [first_idx, second_idx, get_idx(word_len_third)];
                                if try_combination(&combination, &mut sorted, & mut res, &mut current_fill_level) {
                                    //println!("try i {}, free {}, combination {:?}", i, free, combination);
                                    break;
                                }
                            }
                            if second_idx < 1 || word_len(second_idx - 1) <= (word_len_third+1) {
                                break;
                            }
                            second_idx = second_idx - 1;
                        };
                        if current_fill_level == TARGET {
                            break;
                        }
                    }

                    while current_fill_level != TARGET {
                        println!("need whitespace");
                        // what a pitty
                        res.push(' ');
                        current_fill_level += 1;
                    }

                    // replace last whitespace with newline
                    res.pop();
                    res.push('\n');

                    // next line
                    current_fill_level = 0;
                }
            }
        }
        res
    }

}