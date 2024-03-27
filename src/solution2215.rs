pub struct Solution {}

impl Solution {
    pub fn new() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];

        find_difference(nums1, nums2);

        fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
            use std::collections::HashSet;
            let mut set1 = nums1.iter().copied().collect::<HashSet<i32>>();
            let mut set2 = nums2.iter().copied().collect::<HashSet<i32>>();

            for num in nums1 {
                set2.remove(&num);
            }

            for num in nums2 {
                set1.remove(&num);
            }

            let a1: Vec<_> = set1.into_iter().collect();
            let a2: Vec<_> = set2.into_iter().collect();
            vec![a1, a2]
        }
    }
}
