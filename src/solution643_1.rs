use std::usize;

pub struct Solution {}

impl Solution {
    pub fn new() {
        let nums = vec![4, 2, 1, 3, 3];
        let k = 2;
        let res = find_max_average(nums, k);

        // println!("{}", res);
        //should be 3

        fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
            let k = k as usize;

            if k == nums.len() {
                let sum: i32 = nums.iter().sum();

                return sum as f64 / k as f64;
            }

            let mut i: usize = 0;

            let mut sum: i32 = nums[i..k].iter().sum();
            let mut max = sum;

            for num in nums[k..].iter() {
                sum = sum - nums[i] + num;
                if sum > max {
                    max = sum
                }
                i += 1;
            }

            let r: f64 = max as f64 / k as f64;

            r
        }
    }
}
