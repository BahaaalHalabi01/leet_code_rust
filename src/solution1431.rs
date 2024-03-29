pub struct Solution {}

// There are n kids with candies. You are given an integer array candies, where each candies[i] represents the number of candies the ith kid has, and an integer extraCandies, denoting the number of extra candies that you have.
//
// Return a boolean array result of length n, where result[i] is true if, after giving the ith kid all the extraCandies, they will have the greatest number of candies among all the kids, or false otherwise.
//
// Note that multiple kids can have the greatest number of candies.
impl Solution {
    pub fn new() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra = 3;

        kids_with_candies(candies, extra);

        fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
            let sum = candies.iter().max();

            let candies: Vec<bool> = candies
                .iter()
                .map(|v| v + extra_candies >= *sum.unwrap())
                .collect();

            candies
        }
    }
}
