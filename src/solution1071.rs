pub struct Solution {}

impl Solution {
    //answer beats 100% of users in runtime, 38.7% in memory
    //i found an anser which uses string replacement to see if the
    //rest of the values are the same, very interesting i did it manually tho sucks
    pub fn new() {
        // let str1 = String::from("TAUXXTAUXXTAUXXTAUXXTAUXX");
        // let str2 = String::from("TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX");
        let str1 = String::from("EFGABC");
        let str2 = String::from("ABC");

        let res = gcd_of_strings(str1, str2);
        println!("res {}", res);

        fn gcd_of_strings(str1: String, str2: String) -> String {
            fn get_gcd(a: usize, b: usize) -> usize {
                let mut a = a;
                let mut b = b;
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                a
            }

            let (l0, l1) = (str1.len(), str2.len());
            let gcd = get_gcd(std::cmp::max(l0, l1), std::cmp::min(l0, l1));

            let mut str0 = str1.chars();
            let mut str1 = str2.chars();
            let mut mx = String::from("");

            loop {
                let tmp = (str0.next_back(), str1.next_back());

                if tmp == (None, None) {
                    break;
                };

                if tmp.0 != tmp.1 {
                    mx = String::from("");
                    break;
                }

                mx.insert(0, tmp.0.unwrap());

                if mx.len() == gcd {
                    if gcd != l0 {
                         
                        let same = str0.collect::<Vec<char>>().chunks(gcd).all(|c| c.iter().collect::<String>() == mx);
                        if !same {
                            mx = String::from("");
                        };
                    }
                    if gcd != l1 {
                        let same = str1.collect::<Vec<char>>().chunks(gcd).all(|c| c.iter().collect::<String>() == mx);
                        if !same {
                            mx = String::from("");
                        };
                    }

                    break;
                }
            }

            mx
        }
    }
}
