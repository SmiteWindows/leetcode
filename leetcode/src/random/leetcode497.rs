// https://leetcode-cn.com/problems/random-point-in-non-overlapping-rectangles/
// Runtime: 12 ms
// Memory Usage: 4.6 MB
use rand::{distributions::WeightedIndex, prelude::*};
struct Solution {
    rects: Vec<Vec<i32>>,
    rng: ThreadRng,
    len: usize,
    dist: WeightedIndex<i32>,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let rng = thread_rng();
        let len = rects.len();
        let weights = rects
            .iter()
            .map(|v| (v[2] - v[0] + 1) * (v[3] - v[1] + 1))
            .collect::<Vec<_>>();
        let dist = WeightedIndex::new(weights).unwrap();
        Self {
            rects,
            rng,
            len,
            dist,
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        let rect = &self.rects[self.rng.sample(&self.dist)];
        let x = self.rng.gen_range(rect[0]..rect[2] + 1);
        let y = self.rng.gen_range(rect[1]..rect[3] + 1);
        vec![x, y]
    }
}
/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */
// random binary_search
#[test]
fn test1_947() {
    use leetcode_prelude::vec2;
    let mut obj = Solution::new(vec2![[1, 1, 5, 5]]);
    let res1 = obj.pick();
    let res2 = obj.pick();
    let res3 = obj.pick();
    println!("{:?}", res1);
    println!("{:?}", res2);
    println!("{:?}", res3);
    println!();
    let mut obj = Solution::new(vec2![[-2, -2, -1, -1], [1, 0, 3, 0]]);
    let res1 = obj.pick();
    let res2 = obj.pick();
    let res3 = obj.pick();
    let res4 = obj.pick();
    let res5 = obj.pick();
    println!("{:?}", res1);
    println!("{:?}", res2);
    println!("{:?}", res3);
    println!("{:?}", res4);
    println!("{:?}", res5);
}
