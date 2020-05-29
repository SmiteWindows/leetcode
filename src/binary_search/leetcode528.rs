// https://leetcode.com/problems/random-pick-with-weight/
// Runtime: 16 ms
// Memory Usage: 4 MB
use rand::{distributions::WeightedIndex, prelude::*};
struct Solution {
    dist: WeightedIndex<i32>,
    rng: ThreadRng,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let rng = rand::thread_rng();
        let dist = WeightedIndex::new(w).unwrap();
        Self { dist, rng }
    }

    fn pick_index(&mut self) -> i32 {
        self.rng.sample(&self.dist) as i32
    }
}
/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
// random binary_search
#[test]
fn test1_528() {
    let mut obj = Solution::new(vec![1]);
    let ret = obj.pick_index();
    println!("{:?}", ret);
    println!();
    let mut obj = Solution::new(vec![1, 3]);
    let ret_1 = obj.pick_index();
    let ret_2 = obj.pick_index();
    let ret_3 = obj.pick_index();
    let ret_4 = obj.pick_index();
    let ret_5 = obj.pick_index();
    println!("{:?}", ret_1);
    println!("{:?}", ret_2);
    println!("{:?}", ret_3);
    println!("{:?}", ret_4);
    println!("{:?}", ret_5);
}
