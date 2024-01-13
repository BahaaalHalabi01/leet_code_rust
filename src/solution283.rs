pub struct Solution {}
// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
//
// Note that you must do this in-place without making a copy of the array.
impl Solution {
    pub fn new() {
        let mut nums = vec![1, 0, 0, 3, 12];
        move_zeroes(&mut nums);
        println!("s {:?}", nums);

        // i did the slow solution first, swapped them, this solution is pretty nice. **FROM LEET
        // CODE**
        // can use nums.retain in rust also, nice feature
        pub fn move_zeroes(nums: &mut Vec<i32>) {
            let mut non_zero = 0;

            let len: usize = nums.len();

            for i_zero in 0..len {
                if nums[i_zero] != 0 {
                    nums[non_zero] = nums[i_zero];
                    non_zero += 1;
                }
            }

            for x in non_zero..len {
                nums[x] = 0;
            }
        }

        //My dumb solution
        // let mut zero_count = 0;
        //             for i in 0..nums.len() - 1 {
        //                 if nums[i] == 0 {
        //                     zero_count += 1;
        //                     nums.swap(i, i + 1)
        //                 }
        //             }
        //
        //             for _ in 0..zero_count - 1 {
        //                 for i in 0..nums.len() - 1 {
        //                     if nums[i] == 0 {
        //                         nums.swap(i, i + 1)
        //                     }
        //                 }
        //             }
    }
}
