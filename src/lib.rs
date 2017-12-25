pub mod book_shorter {

    pub fn number_words(book_in: &str) -> usize {
        book_in.split_whitespace().count()
    }

    pub fn longest_word(book_in: &str) -> usize {
        book_in.split_whitespace().map(|x| x.chars().count()).max().unwrap_or(0)
    }

    pub fn hist_word(book_in: &str) -> Vec<u32> {
        let mut vec = Vec::<u32>::new();
        for x in book_in.split_whitespace() {
            let cnt = x.chars().count();
            if vec.len() < cnt {
                vec.resize(cnt, 0)
            }
            vec[cnt-1] = vec[cnt-1] + 1;
        }
        vec
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