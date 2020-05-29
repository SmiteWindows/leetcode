// https://leetcode.com/problems/random-pick-with-blacklist/
use rand::{distributions::Uniform, prelude::*};
use std::collections::{HashMap, HashSet};
struct Solution {
    rng: ThreadRng,
    m: HashMap<i32, i32>,
    wlen: i32,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let rng = thread_rng();
        let mut m = HashMap::new();
        let wlen = n - blacklist.len() as i32;
        let mut w = HashSet::new();
        for i in wlen..n {
            w.insert(i);
        }
        for x in blacklist.iter() {
            w.remove(x);
        }
        for &x in blacklist.iter() {
            if x < wlen {
                m.insert(x, *w.iter().next().unwrap());
            }
        }
        Self { rng, m, wlen }
    }

    fn pick(&mut self) -> i32 {
        let distribution = Uniform::new(0, self.wlen + 1);
        let k = self.rng.sample(distribution);
        *self.m.entry(k).or_insert(k)
    }
}
/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(N, blacklist);
 * let ret_1: i32 = obj.pick();
 */
// random sort binary_search hash_table
#[test]
#[ignore]
fn test1_710() {
    let mut obj = Solution::new(1, vec![]);
    let ret_1 = obj.pick();
    let ret_2 = obj.pick();
    let ret_3 = obj.pick();
    println!("{:?}", ret_1);
    println!("{:?}", ret_2);
    println!("{:?}", ret_3);
    let mut obj = Solution::new(2, vec![]);
    let ret_1 = obj.pick();
    let ret_2 = obj.pick();
    let ret_3 = obj.pick();
    println!("{:?}", ret_1);
    println!("{:?}", ret_2);
    println!("{:?}", ret_3);
    let mut obj = Solution::new(3, vec![1]);
    let ret_1 = obj.pick();
    let ret_2 = obj.pick();
    let ret_3 = obj.pick();
    println!("{:?}", ret_1);
    println!("{:?}", ret_2);
    println!("{:?}", ret_3);
    let mut obj = Solution::new(4, vec![2]);
    let ret_1 = obj.pick();
    let ret_2 = obj.pick();
    let ret_3 = obj.pick();
    println!("{:?}", ret_1);
    println!("{:?}", ret_2);
    println!("{:?}", ret_3);
}
