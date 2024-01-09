pub struct Solution {}

impl Solution {
    pub fn new() {
        let str1 = String::from("TAUXXTAUXXTAUXXTAUXXTAUXX");
        let str2 = String::from("TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX");
        // let str1 = String::from("EFGABC");
        // let str2 = String::from("ABC");

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
                    if str0.next_back().is_some() {
                        let mut str0 = str0.collect::<Vec<char>>();

                        str0.insert(0, tmp.0.unwrap());

                        println!("mx {:?}",str0);
                        let same = str0.chunks(gcd).all(|c| c.iter().collect::<String>() == mx);

                        match same {
                            true => break,
                            false => {
                                mx = String::from("");
                                break;
                            }
                        };
                    }

                    if str1.next_back().is_some() {
                        let mut str1 = str1.collect::<Vec<char>>();
                        str1.insert(0, tmp.1.unwrap());
                        println!("mx {}",mx);
                        let same = str1.chunks(gcd).all(|c| c.iter().collect::<String>() == mx);

                        match same {
                            true => break,
                            false => {
                                mx = String::from("");
                                break;
                            }
                        };
                    }
                    //check the remaining values

                    break;
                }
            }

            mx
        }
    }
}
