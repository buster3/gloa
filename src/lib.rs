pub mod book_shorter {

    pub fn number_words(book_in: &str) -> usize {
        book_in.split_whitespace().count()
    }

    pub fn longest_word(book_in: &str) -> usize {
        book_in
            .split_whitespace()
            .map(|x| x.chars().count())
            .max()
            .unwrap_or(0)
    }

    pub fn hist_word(book_in: &str) -> Vec<u32> {
        let mut vec: Vec<u32> = Vec::new();
        for x in book_in.split_whitespace() {
            let cnt = x.chars().count();
            if vec.len() < cnt {
                vec.resize(cnt, 0)
            }
            vec[cnt - 1] = vec[cnt - 1] + 1;
        }
        vec
    }

    pub const TARGET: isize = 81;

    struct Word<'a> {
        word: &'a str,
        chars: u8,
    }

    fn word_len(i : usize) -> isize {
        i as isize + 2
    }

    fn get_idx(i : isize) -> usize {
        assert!(i >= 2);
        i as usize - 2
    }

    pub fn compress(book_in: &str) -> Vec<isize> {
        let mut sorted = hist_word(book_in);
        let mut res: Vec<isize> = Vec::new();
        res.push(0);
        let mut out_idx = 0 as usize;

        for i in (0..sorted.len()).rev() {
            while sorted[i] > 0 {

                let mut free = TARGET - (res[out_idx] as isize);
                let border = word_len(i);

                if (free - word_len(i)) > border {
                    // fill up
                    res[out_idx] = res[out_idx] + word_len(i);
                    sorted[i] = sorted[i] - 1;
                    free = free - word_len(i);
                } else {
                    // fill with two words
                    let mut current = get_idx(free / 2);
                    while current <= i && free - word_len(current) >= 2 {
                        let second_idx = get_idx(free - word_len(current));
                        if sorted[current] > 0 && sorted[second_idx] > 0 {
                            // found a pair to fill the line
                            res[out_idx] = res[out_idx] + word_len(current) + word_len(second_idx);
                            sorted[current] = sorted[current] - 1;
                            sorted[second_idx] = sorted[second_idx] - 1;
                            break;
                        }
                        current = current + 1;
                    }
                    if res[out_idx] != TARGET {
                        // too bad...
                        println!("miss i {}, free {}, sorted {:?}", i, free, sorted);
                    }

                    // next line
                    res.push(0);
                    out_idx = out_idx + 1;

                }



            }
        }

        res
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        static DUMMY_DATA: &'static str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
bbbbbbbbbbbbbbbbbbbbbbbbbbbb
cccccccccccccccccccccccccccccccc
eeeeeeeeeeeeeeeeeeeeeeee
ddddddddddddddddddddddddddddd
fffffffffffffffffff
gggg
h";

        #[test]
        fn cnt_words() {
            assert_eq!(8, number_words(DUMMY_DATA));
        }

        #[test]
        fn longest_word_test() {
            assert_eq!(36, longest_word(DUMMY_DATA));
        }

    }

}