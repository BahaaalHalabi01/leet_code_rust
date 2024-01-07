use std::collections::HashSet;

pub struct Solution {}

// Given two arrays of unique digits nums1 and nums2, return the smallest number that contains at least one digit from each array.
//
//
impl Solution {

    //easy solution but i need to make less mistakes
    //@todo check again 
    pub fn new() {
        let nums1 = vec![4,1,3];
        let nums2 = vec![5,7];

        let x = min_number(nums1, nums2);
        println!("x {}", x);
        fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
            let set: HashSet<i32> = nums1.into_iter().collect();
            let mut min = 10;
            let mut min2 = 10;

            for &num in &nums2 {
                if num < min2 {
                    min2 = num
                }
                if set.contains(&num) && num < min {
                    min = num;
                }
            }

            if min != 10 {
                return min;
            }

            let min1 = set.into_iter().min().unwrap();
            println!(" min {} min {}",min1,min2);
            if min1 > min2 {
                min2 * 10 + min1
            } else {
                min1 * 10 + min2
            }
        }
    }
}
