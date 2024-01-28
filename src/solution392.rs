use std::{collections::HashMap, path::is_separator};

pub struct Solution {}
// Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
//
// A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
//
impl Solution {
    pub fn new() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        let s = is_subsequence(s, t);
        println!("return {}", s);
        pub fn is_subsequence(s: String, t: String) -> bool {
            let mut map: HashMap<char, Vec<usize>> = HashMap::new();
            for (i, v) in t.char_indices() {
                if map.contains_key(&v) {
                    let indicies = map.get_mut(&v).unwrap();
                    indicies.push(i);
                    continue;
                }
                map.insert(v, vec![i]);
            }

            println!("map {:?}", map);

            let mut sub = false;
            let mut i = 0;
            let s = s.as_bytes();
            while i < s.len() {
                let mut index_first = 0;
                let mut index_second = 0;
                let v = char::from(*s.get(i).unwrap());
                let x = char::from(*s.get(i + 1).unwrap());
                let indicies_first = map.get(&v);
                let indicies_second = map.get(&v);
                match indicies_first {
                    Some(x) => index_first = x[0],
                    None => return false,
                }
                match indicies_second {
                    Some(x) => index_second = x[0],
                    None => return false,
                }

                if index_first > index_second {}

                i += 1;
            }

            true

            // int i=0,j=0;
            // int n = s.size();
            // int m = t.size();
            // while(i<n && j<m){
            //     if(s[i] == t[j]){
            //         i++,j++;
            //     }else{
            //         j++;
            //     }
            // }
            //
            // if(i==n) return true;
            // return false;
            // }
        }
    }
}
