pub mod book_info;

pub mod book_shorter {

    pub const TARGET: isize = 81;

    pub struct LineIterator<'a> {
        sorted: Vec<Vec<&'a str>>,
    }

    impl<'a> LineIterator<'a> {

        /// Constructs a new Line Iterator
        ///
        /// book_in: string containing the book to be reformated
        pub fn new(book_in: & str) -> LineIterator {
            let mut vec: Vec<Vec<&str>> = vec![Vec::new()];
            for x in book_in.split_whitespace() {
                let cnt = x.chars().count();
                while vec.len() < cnt {
                    vec.push(Vec::new())
                }
                vec[cnt - 1].push(x);
            }
            LineIterator { sorted: vec }
        }
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

    impl<'a> Iterator for LineIterator<'a> {
        type Item = String;

        /// get next reformated line
        fn next(& mut self) -> Option<String> {
            let mut fill_level = 0;
            let mut res = String::new();
            for i in (0..self.sorted.len()).rev() {
                while self.sorted[i].len() > 0 {
                    match fill_line(&mut self.sorted, i, fill_level, &mut res) {
                        FillResult::Done => {
                            // good
                            while fill_level > 0 {
                                res.push(' ');
                                fill_level -= 1;
                            }
                            // replace last whitespace with newline
                            res.pop();
                            res.push('\n');
                            return Some(res);
                        }
                        FillResult::Failed(_) => {
                            // filed to find a optimal solution. Find a good suboptimal solution
                            fill_level += 1;
                            println!("Failed to fill a line");
                        }
                    }
                }
            }
            None
        }
    }

    fn fill_line(sorted: &mut Vec<Vec<&str>>,
                 in_idx: usize,
                 current_fill_level: isize,
                 res: &mut String)
                 -> FillResult {
        let mut idx = in_idx;
        let free = TARGET - current_fill_level;

        while sorted[idx].len() == 0 && idx > 0 {
            idx -= 1;
        }

        if sorted[idx].len() == 0 {
            // done, no more data
            return FillResult::Done;
        }

        if free <= word_len(idx) {
            // do a optimal fillup
            let required = free;
            if required >= 2 && sorted[get_idx(required)].len() > 0 {
                // done - we filled a line
                res.push_str(sorted[get_idx(required)].pop().unwrap());
                res.push(' ');
                return FillResult::Done;
            } else {
                // no optimal solution possible
                return FillResult::Failed(required);
            }
        } else {
            loop {
                let candidate = sorted[idx].pop().unwrap();
                match fill_line(sorted, idx, current_fill_level + word_len(idx), res) {
                    FillResult::Done => {
                        res.push_str(candidate);
                        res.push(' ');
                        return FillResult::Done;
                    }
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
                        assert!(sorted[idx].len() != 0);
                    }
                }
            }
        }
    }
}