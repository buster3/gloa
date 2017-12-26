pub mod book_shorter {

    pub fn number_words(book_in: &str) -> usize {
        book_in.split_whitespace().count()
    }

    pub fn longest_word(book_in: &str) -> usize {
        book_in.split_whitespace().map(|x| x.chars().count()).max().unwrap_or(0)
    }

    pub fn hist_word(book_in: &str) -> Vec<u32> {
        let mut vec : Vec<u32> = Vec::new();
        for x in book_in.split_whitespace() {
            let cnt = x.chars().count();
            if vec.len() < cnt {
                vec.resize(cnt, 0)
            }
            vec[cnt-1] = vec[cnt-1] + 1;
        }
        vec
    }

    pub const TARGET : usize = 81;

    struct Word<'a> {
        word: &'a str,
        chars: u8,
    }

    pub fn compress(book_in: &str) -> Vec<u32> {
        let mut sorted  = hist_word(book_in);
        let mut res : Vec<u32> = Vec::new();
        res.push(0);
        let mut idx = 0 as usize;

        for i in (0..sorted.len()).rev() {
            let word_len = i + 1 + 1;

            while sorted[i] > 0 {
                let mut free = TARGET - (res[idx] as usize);
                let mut wanted = free;
            
                // fill up with the big ones
                if free == word_len || free >= (word_len + 2) {
                    res[idx] = res[idx] + word_len as u32;
                    free = free - word_len;
                    wanted = free;
                    sorted[i] = sorted[i] - 1;
                } else {
                    // use what fits
                    while wanted > 1 {
                        if sorted[wanted - 2] > 0 {
                            res[idx] = res[idx] + wanted as u32;
                            sorted[wanted-2] = sorted[wanted-2] - 1;
                            free = free - wanted;
                            wanted = free;
                        } else {
                            if wanted == free {
                                wanted = wanted - 2;
                            } else {
                                wanted = wanted - 1;
                            }
                            
                        }
                    }
                }
                if wanted <= 1 && free != 0 {
                    println!("debug");
                }

                if wanted <= 1 {
                    res.push(0);
                    idx = idx + 1;
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