pub struct Solution {}
// You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.
//
// Return the merged string.
impl Solution {
    pub fn new() {
        let word1 = String::from("abcd");
        let word2 = String::from("pq");

        let v = merge_alternately(word1, word2);

        println!("v {}", v);

        fn merge_alternately(word1: String, word2: String) -> String {
            let mut word1 = word1.chars().peekable();
            let mut word2 = word2.chars().peekable();
            let mut value = String::from("");

            loop {
                if  (word1.peek().is_some(), word2.peek().is_some()) == (false, false) {
                    break;
                }

                match word1.next() {
                    Some(x) => {
                        println!("x {}", x);
                        value.push(x);
                    }
                    None => {}
                }
                match word2.next() {
                    Some(x) => {
                        println!("x {}", x);
                        value.push(x)
                    }
                    None => {}
                }
            }
            value
        }
    }
}
