fn main() {
    solution2553()
}

fn solution2553() {
    // Given an array of positive integers nums, return an array answer that consists of the digits of each integer in nums after separating them in the same order they appear in nums.
    //
    // To separate the digits of an integer is to get all the digits it has in the same order.
    //
    //     For example, for the integer 10921, the separation of its digits is [1,0,9,2,1].

    struct Solution {}

    impl Solution {
        pub fn separate_digits(nums: &mut Vec<i32>) {
            if nums.len() > 1000 || nums.len() < 1 {
                println!("Unsuported size of array");
                panic!()
            }

            for num in nums.clone().iter() {
                let char = num.to_string();
                split_and_delete(char, nums);
                nums.remove(0);
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
        }
    }

    let mut input = Vec::from([7, 1, 3, 0]);
    Solution::separate_digits(&mut input);

    println!("input {:?}", input)
}
