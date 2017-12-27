pub mod book_shorter {
    use std::cmp;

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

    pub const MAX_COMBINATIONS: usize = 3;

    fn try_combination(combination: &Vec<usize>, sorted: &mut Vec<u32>, res: &mut isize) -> bool {
        let mut unique : Vec<usize> = combination.clone();
        unique.dedup();
        let combination_possible = unique.iter().all(|&x| {
            let cnt = combination.iter().fold(0, |total, &s| { 
            if s == x {
                total + 1
            } else {
                total
            }});
            sorted[x] >= cnt as u32
        });

        if combination_possible {
            // found a pair to fill the line
            *res += combination.iter().map(|&x| word_len(x)).sum();
            for &x in combination.iter() {
                sorted[x] -= 1;
            }
            true
        } else {
            false
        }
    }

    pub fn compress(book_in: &str) -> Vec<isize> {
        let mut sorted = hist_word(book_in);
        let mut res: Vec<isize> = Vec::new();
        res.push(0);
        let mut out_idx = 0 as usize;

        for i in (0..sorted.len()).rev() {
            while sorted[i] > 0 {

                let mut free = TARGET - res[out_idx];

                let mut dummy = MAX_COMBINATIONS as isize;
                let mut border = 0;
                let mut i_t = i;
                while dummy > 0 {
                    let available = cmp::min(dummy, sorted[i_t] as isize);
                    border += word_len(i_t) * available;
                    dummy -= available;
                    if i_t > 0 {
                        i_t -= 1;
                    } else {
                        break;
                    }

                }

                //let border = word_len(i) * MAX_COMBINATIONS as isize;

                // TODO greater equal?
                if free > border {
                    // fill up
                    res[out_idx] += word_len(i);
                    sorted[i] -= 1;
                } else {
                    // try fill up with three more words
                    for k in 0..2 {
                        let first_idx = i - k;
                        let to_be_filled = free - word_len(first_idx);
                        // ceil idx
                        let mut second_idx = get_idx((to_be_filled + 1) / 2);
                        while second_idx <= i && to_be_filled - word_len(second_idx) >= 2 {
                            let thrid_idx = get_idx(to_be_filled - word_len(second_idx));
                            let combination = vec![first_idx, second_idx, thrid_idx];
                            if try_combination(&combination, &mut sorted, &mut res[out_idx]) {
                                //println!("try i {}, free {}, combination {:?}, sorted {:?}", i, free, combination, sorted);
                                break;
                            }
                            second_idx = second_idx + 1;
                        }
                        if res[out_idx] == TARGET {
                            break;
                        }
                    }
                    if res[out_idx] != TARGET {
                        // soo bad... :(
                        println!("miss i {}, free {}, sorted {:?}", i, free, sorted);
                        if free >= word_len(i) {
                            // fill up
                            println!("fill up");
                            res[out_idx] += word_len(i);
                            sorted[i] -= 1;
                            //continue;
                        } else {
                            println!("fill up not possible");
                        }
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