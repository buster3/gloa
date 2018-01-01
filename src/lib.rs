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

    enum FillResult {
        Done,
        Failed(isize),
    }

    fn word_len(i: usize) -> isize {
        i as isize + 2
    }

    fn get_idx(i: isize) -> usize {
        assert!(i >= 2);
        i as usize - 2
    }

    pub const MAX_COMBINATIONS: usize = 3;


    fn fill_line(sorted: & mut Vec<Vec<Word>>, in_idx: usize, current_fill_level: isize, res: &mut String) -> FillResult {
        let mut idx = in_idx;
        let free = TARGET - current_fill_level;
        while sorted[idx].len() == 0 && idx > 0 {
            idx -= 1;
        }

        if sorted[idx].len() == 0 {
            // done, no more data
            return FillResult::Done
        }

        if free <= word_len(idx) {
            // do a optimal fillup
            let required = free;
            if required >= 2 && sorted[get_idx(required)].len() > 0 {
                // done - we filled a line
                res.push_str(sorted[get_idx(required)].pop().unwrap().w);
                res.push(' ');
                return FillResult::Done
            } else {
                // not a possible solution
                return FillResult::Failed(required)
            }
        } else {
            loop {
                let candidate = sorted[idx].pop().unwrap();
                match fill_line(sorted, idx, current_fill_level + word_len(idx), res) {
                    FillResult::Done => {
                        res.push_str(candidate.w);
                        res.push(' ');
                        return FillResult::Done;
                    },
                    FillResult::Failed(sub_word_len) => {
                        // put candidate back
                        sorted[idx].push(candidate);

                        if word_len(idx) > sub_word_len {
                            idx -= 1;
                            while sorted[idx].len() == 0 && idx > 0 {
                                idx -= 1;
                            }
                            if sorted[idx].len() == 0 {
                                return FillResult::Failed(sub_word_len);
                            }
                        } else {
                            return FillResult::Failed(sub_word_len);
                        }
                        assert!( sorted[idx].len() != 0);
                    }
                }
            };
        }
    }

    pub fn compress(book_in: &str) -> String {
        let mut sorted = load_data(book_in);
        let mut res: String = String::new();

        let mut fill_level = 0;
        for i in (0..sorted.len()).rev() {
            while sorted[i].len() > 0 {
                match fill_line( &mut sorted, i, fill_level, &mut res) {
                    FillResult::Done => {
                        // good
                        while fill_level > 0 {
                            res.push(' ');
                            fill_level -= 1;
                        }
                        // replace last whitespace with newline
                        res.pop();
                        res.push('\n');
                    },
                    FillResult::Failed(sub_word_len) => {
                        // wo found no optimal solution to fill the line, try with an unfilled character
                        fill_level += 1;
                        println!("Failed to fill a line");
                    }
                }
            }
        }
        res
    }
}