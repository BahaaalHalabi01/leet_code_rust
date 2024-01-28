pub struct Solution {}

impl Solution {
    pub fn new() {
        let s = String::from("");
        let t = String::from("ahbgdc");
         is_subsequence(s, t);

        fn is_subsequence(s: String, t: String) -> bool {
            let mut i: usize = 0;
            let s: Vec<char> = s.chars().collect();

            let mut j: usize = 0;
            let t: Vec<char> = t.chars().collect();

            if s.len() ==0 {return true};
            if t.len() ==0 {return false};

            loop {
                if t[j] == s[i] {
                    i += 1;
                }
                j += 1;

                if i == s.len() {
                    return true;
                };
                if j == t.len() {
                    break;
                }
            }

            false
        }
    }
}
