pub struct Solution {}

// There is a bag that consists of items, each item has a number 1, 0, or -1 written on it.
//
// You are given four non-negative integers numOnes, numZeros, numNegOnes, and k.
//
// The bag initially contains:
//
//     numOnes items with 1s written on them.
//     numZeroes items with 0s written on them.
//     numNegOnes items with -1s written on them.
//
// We want to pick exactly k items among the available items. Return the maximum possible sum of numbers written on the items.

impl Solution {
    pub fn new() {
        let num_ones = 3;
        let num_zeros = 2;
        let num_neg_ones = 1;
        let k = 5;

        let x = k_items_with_maximum_sum(num_ones, num_zeros, num_neg_ones, k);

        println!("x {}", x);

        fn k_items_with_maximum_sum(
            num_ones: i32,
            num_zeros: i32,
            num_neg_ones: i32,
            k: i32,
        ) -> i32 {
            let mut k_elements: Vec<i32> = vec![];

            for _ in 0..num_ones {
                k_elements.push(1)
            }
            for _ in 0..num_zeros {
                k_elements.push(0)
            }
            for _ in 0..num_neg_ones {
                k_elements.push(-1)
            }

            let mut sum = 0;
            for i in 0..k {
                sum += k_elements[i as usize]
            }
            sum
        }
    }
}
