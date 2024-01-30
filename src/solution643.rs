use std::ops::Div;

pub struct Solution {}

impl Solution {
    pub fn new() {
        let nums = vec![-1];
        let k = 1;
        let r = find_max_average(nums, k);
        println!("result {}", r);
        //sliding window
        fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
            let mut max: f64 = f64::MIN;

            if k as usize > nums.len() {
                return max;
            };

            if k as usize == nums.len() {
                return nums.iter().sum::<i32>() as f64 / k as f64;
            }

            let mut i = 0 as usize;
            let mut j = (k - 1) as usize;

            while j < nums.len() {
                let n = nums[i..=j].iter();

                let current_max = f64::from(n.sum::<i32>() as f64 / k as f64);

                if current_max > max {
                    max = current_max
                };

                i += 1;
                j += 1;
            }

            max
        }
    }
}
