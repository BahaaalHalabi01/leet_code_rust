// There are two mice and n different types of cheese, each type of cheese should be eaten by exactly one mouse.
//
// A point of the cheese with index i (0-indexed) is:
//
//     reward1[i] if the first mouse eats it.
//     reward2[i] if the second mouse eats it.
//
// You are given a positive integer array reward1, a positive integer array reward2, and a non-negative integer k.
//
// Return the maximum points the mice can achieve if the first mouse eats exactly k types of cheese.

pub struct Solution {}

impl Solution {
    //took me a long time
    //feels bad solution
    //@todo try again similar <strong>greedy</strong> problems
    pub fn new() {
        // let reward1 = vec![1, 4, 4, 6, 4];
        // let reward2 = vec![6, 5, 1, 6, 3];
        // k 1
        //expected 24

        // let reward1 = vec![1, 2, 1, 2, 1, 2];
        // let reward2 = vec![2, 1, 1, 2, 2, 1];
        // k 0
        //expected 9
        //

        let reward1 = vec![1];
        let reward2 = vec![4];
        // k 1

        let v = mice_and_cheese(reward1, reward2, 1);
        println!("return {}", v);

        pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
            let mut sum = 0;
            let mut diff: Vec<(i32, usize)> = reward1
                .iter()
                .zip(reward2.iter())
                .enumerate()
                .map(|(i, (&a, &b))| {
                    sum += b;
                    (a - b, i)
                })
                .collect();

            println!("sum {}", sum);
            diff.sort_by_key(|&(diff, _)| std::cmp::Reverse(diff));
            diff.truncate(k as usize);

            for (score, index) in diff {
                println!(" s {} i {} v {}", score, index, reward2[index]);
                sum += reward1[index] - reward2[index]
            }

            sum
        }
    }

    //the good solution
    //
    // pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
    //     let mut reward = reward1.into_iter().zip(reward2.into_iter()).collect::<Vec<_>>();
    //     reward.sort_unstable_by_key(|x| x.1 - x.0);
    //     let (mut ret, k, n) = (0, k as usize, reward.len());
    //     for (i, (first, second)) in reward.into_iter().enumerate() {
    //         ret += if i < k { first } else { second };
    //     }
    //     ret
    // }
}
