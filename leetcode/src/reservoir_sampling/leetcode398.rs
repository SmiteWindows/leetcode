// https://leetcode.com/problems/random-pick-index/
// Runtime: 12 ms
// Memory Usage: 4.3 MB
use rand::prelude::*;
struct Solution {
    nums: Vec<i32>,
    rng: ThreadRng,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let rng = thread_rng();
        Self { nums, rng }
    }

    fn pick(&mut self, target: i32) -> i32 {
        let mut count = 0;
        let mut res = 0;
        for (i, &num) in self.nums.iter().enumerate() {
            if num == target {
                count += 1;
                if self.rng.gen_range(0, count) == 0 {
                    res = i;
                }
            }
        }
        res as i32
    }
}
/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
// reservoir_sampling
#[test]
fn test1_398() {
    let nums = vec![1, 2, 3, 3, 3];
    let mut obj = Solution::new(nums);
    for _ in 0..100 {
        let res1 = obj.pick(3);
        match res1 {
            2 => print!("2"),

            3 => print!("3"),

            4 => print!("4"),

            _ => panic!(),
        }
    }
    let res2 = obj.pick(1);
    assert_eq!(res2, 0);
}
