// https://leetcode-cn.com/problems/shuffle-an-array/
// Runtime: 28 ms
// Memory Usage: 5.4 MB
use rand::prelude::*;
struct Solution {
    rng: ThreadRng,
    nums: Vec<i32>,
    n: usize,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let rng = thread_rng();
        Self { rng, nums, n }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.nums.to_vec()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        let mut v = self.nums.to_vec();
        let n = self.n;
        for i in 0..n {
            let j = self.rng.gen_range(i..n);
            v.swap(i, j);
        }
        v
    }
}
/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */
#[test]
fn test384() {
    // Init an array with set 1, 2, and 3.
    let nums = vec![1, 2, 3];
    let mut solution = Solution::new(nums);

    // Shuffle the array [1,2,3] and return its result. Any permutation of [1,2,3] must equally likely to be returned.
    solution.shuffle();

    // Resets the array back to its original configuration [1,2,3].
    assert_eq!(solution.reset(), vec![1, 2, 3]);

    // Returns the random shuffling of array [1,2,3].
    solution.shuffle();
}
