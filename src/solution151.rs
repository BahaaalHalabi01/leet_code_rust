pub struct Solution {}

impl Solution {
    pub fn new() {
        let s = String::from("the sky is blue");
        let v = reverse_words(s);
        println!("v {}", v);

        pub fn reverse_words(s: String) -> String {

            // rust is pretty strong
            return s
                .split_ascii_whitespace()
                .rev()
                .collect::<Vec<_>>()
                .join(" ");

            let s = s.trim().chars();
            let mut v = String::from("");
            let mut was_space = false;
            let mut v_words: Vec<String> = vec![];
            for x in s {
                // i trimmed first isn't a space;

                if !x.is_whitespace() {
                    v.push(x);
                    was_space = false
                }

                if x.is_whitespace() && !was_space {
                    was_space = true;
                    v_words.insert(0, v);
                    v = String::from("");
                }
            }

            if v.len() > 0 {
                v_words.insert(0, v);
            }

            v_words.join(" ")
        }
    }
}
