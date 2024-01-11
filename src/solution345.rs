pub struct Solution {}

impl Solution {
    pub fn new() {
        let s = String::from("eaa");
        let s = reverse_vowels(s);
        println!(" v {}", s);

        fn reverse_vowels(s: String) -> String {
            if s.len() == 1 {
                return s;
            };

            let mut z: usize = s.len() - 1;
            let mut i: usize = 0;
            let mut s: Vec<char> = s.chars().collect();
            let vowels = vec!["a", "e", "i", "o", "u"];
            loop {
                if i >= z {
                    break;
                }

                let first = s[i].to_lowercase().to_string();
                let first = first.as_str();
                let first: bool = vowels.contains(&first);
                let last = s[z].to_lowercase().to_string();
                let last = last.as_str();
                let last = vowels.contains(&last);

                if first && last {
                    s.swap(i, z);
                    i += 1;
                    z -= 1;
                    continue;
                }
                if first {
                    z -= 1;
                    continue;
                }
                if last {
                    i += 1;
                    continue;
                }

                i += 1;
                z -= 1;
            }

            s.into_iter().collect::<String>()

        }
    }
}
