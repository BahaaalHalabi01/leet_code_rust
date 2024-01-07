use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

mod solution2610;
mod solution2611;

fn main() {
    // solution2553()
    // solution2554()
    // solution2549()
    // solution2610::Solution::new();
     solution2611::Solution::new();
}

fn solution2549() {
    // You are given a positive integer n, that is initially placed on a board. Every day, for 109 days, you perform the following procedure:
    //
    //     For each number x present on the board, find all numbers 1 <= i <= n such that x % i == 1.
    //     Then, place those numbers on the board.
    //
    // Return the number of distinct integers present on the board after 109 days have elapsed.
    //
    struct Solution {}
    impl Solution {
        pub fn distinct_integers(n: i32) -> i32 {
            // Time to quit programming wow
            // if n > 1 {
            //     n - 1
            // } else {
            //     1
            // }

            let mut numbers: HashSet<i32> = HashSet::new();
            numbers.insert(n);
            let mut index = 2;

            loop {
                if index > n {
                    break;
                }

                for n in numbers.clone() {
                    for j in 2..n {
                        if n % j == 1 {
                            numbers.insert(j);
                        }
                    }
                }
                index += 1;
            }

            println!(" numbers {:?}", numbers);
            numbers.len() as i32
        }
    }

    Solution::distinct_integers(2);
}

fn solution2554() {
    // You are given an integer array banned and two integers n and maxSum. You are choosing some number of integers following the below rules:
    //
    //     The chosen integers have to be in the range [1, n].
    //     Each integer can be chosen at most once.
    //     The chosen integers should not be in the array banned.
    //     The sum of the chosen integers should not exceed maxSum.
    //
    // Return the maximum number of integers you can choose following the mentioned rules.
    //

    struct Solution {}

    impl Solution {
        pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
            let banned = banned.into_iter().collect::<HashSet<_>>();

            let nums = (1..=n).filter(|v| !banned.contains(v));
            let mut sum = 0;
            let mut ans = 0;

            for n in nums {
                sum += n;
                if sum > max_sum {
                    break;
                }
                ans += 1;
            }

            ans
        }
    }

    let banned = vec![11];
    let result = Solution::max_count(banned, 7, 50);
    println!("result {}", result);
}

fn solution2553() {
    // Given an array of positive integers nums, return an array answer that consists of the digits of each integer in nums after separating them in the same order they appear in nums.
    //
    // To separate the digits of an integer is to get all the digits it has in the same order.
    //
    //     For example, for the integer 10921, the separation of its digits is [1,0,9,2,1].

    struct Solution {}

    impl Solution {
        pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
            let mut seperated_nums: Vec<i32> = nums.clone();

            if nums.len() > 1000 || nums.len() < 1 {
                println!("Unsuported size of array");
                panic!()
            }

            for num in nums.clone().iter() {
                let char = num.to_string();
                split_and_delete(char, &mut seperated_nums);
                seperated_nums.remove(0);
            }

            fn split_and_delete(number: String, nums: &mut Vec<i32>) {
                println!("number {}", number);
                if !number.chars().all(char::is_numeric) {
                    //not a numeric string
                    panic!()
                }

                let postive_number: i32 = number.parse().unwrap();
                if postive_number <= 0 || postive_number > i32::pow(10, 5) {
                    // less than 0
                    panic!()
                }

                for num in number.chars() {
                    //i am sure the numbers are numeric,postive from above
                    let parsed = num.to_digit(10).unwrap() as i32;
                    nums.push(parsed);
                }
            }

            println!("input {:?}", seperated_nums);

            return seperated_nums;
        }
    }

    let input = Vec::from([7, 1, 3, 0]);
    let input1 = Vec::from([13, 25, 83, 77]);
    Solution::separate_digits(input);
    Solution::separate_digits(input1);

    //fast solution for refernce **NOT MINE**
    //
    // pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    //     let mut rs = vec![];
    //     for &n in &nums {
    //         let mut n = n;
    //         let mut tp = vec![];
    //         while n > 0 {
    //             tp.push(n % 10);
    //             n /= 10;
    //         }
    //         tp.reverse();
    //         rs.append(&mut tp);
    //     }
    //     rs
    // }
}
