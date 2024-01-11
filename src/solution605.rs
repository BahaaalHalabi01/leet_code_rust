pub struct Solution {}

// You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.
//
// Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.

impl Solution {
    pub fn new() {
        let flowerbed = vec![0,0,1];
        let n = 2;
        let v = can_place_flowers(flowerbed, n);
        println!(" v {}", v);

        fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
            let len = flowerbed.len();
            let mut count: usize = 0;
            let mut flowerbed = flowerbed;

            for i in 0..len {
                if count == n as usize {
                    break;
                }

                if i == 0 {
                    if flowerbed[i] == 0 && (i == len - 1 || flowerbed[i + 1] == 0) {
                        flowerbed[i] = 1;
                        count += 1;
                        continue;
                    }
                    continue;
                }

                if i == len - 1 {
                    if flowerbed[i] == 0 && flowerbed[i - 1] == 0 {
                        flowerbed[i] = 1;
                        count += 1;
                        break;
                    }
                    break;
                }

                if flowerbed[i - 1] == 0 && flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
                    flowerbed[i] = 1;
                    count += 1;
                }
            }
            println!("v {}", count);

            count == n as usize
        }
    }
}
