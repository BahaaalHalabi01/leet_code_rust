use std::collections::{HashMap, HashSet};

// You are given an integer array nums. You need to create a 2D array from nums satisfying the following conditions:
//
//     The 2D array should contain only the elements of the array nums.
//     Each row in the 2D array contains distinct integers.
//     The number of rows in the 2D array should be minimal.
//
// Return the resulting array. If there are multiple answers, return any of them.
//
// Note that the 2D array can have a different number of elements on each row.
//

pub struct Solution {}

impl Solution {
    pub fn new() {
        find_matrix(vec![1, 3, 4, 1, 2, 3, 1]);

        fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
            //imba solution beats 100% of users!!!!!
            let mut nums_matrix: Vec<Vec<i32>> = vec![vec![]];

            let mut map: HashMap<i32, i32> = HashMap::new();

            let mut outer_index = 0;
            for &n in &nums {
                let count = map.entry(n).or_insert(0);
                *count += 1;

                println!("count {} num {}", count, n);
                if count == &1 {
                    //first time i saw
                    if outer_index != 0 {
                        outer_index = 0
                    };
                    nums_matrix[outer_index].push(n);
                } else {
                    println!("num {}", &n);
                    if *count as usize > nums_matrix.len() {
                        nums_matrix.push(vec![])
                    }
                    outer_index = *count as usize - 1;
                    nums_matrix[outer_index].push(n);
                }
            }

            println!("matrix {:?}", nums_matrix);
            nums_matrix
        }
    }
}
