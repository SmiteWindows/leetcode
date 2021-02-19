// https://leetcode-cn.com/problems/maximum-performance-of-a-team/
// Runtime: 16 ms
// Memory Usage: 4 MB
use std::{cmp::Reverse, collections::BinaryHeap};
const MOD: i64 = 1_000_000_007;
pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut max_efficiency: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    for i in 0..n {
        max_efficiency.push((efficiency[i], speed[i]));
    }
    let mut min_speed: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut sum_speed = 0;
    let mut res = 0;
    while let Some((e, s)) = max_efficiency.pop() {
        sum_speed += s as i64;
        min_speed.push(Reverse(s));
        if min_speed.len() > k {
            sum_speed -= min_speed.pop().unwrap().0 as i64;
        }
        res = res.max(sum_speed as i64 * e as i64);
    }
    (res % MOD) as i32
}
// sort greedy
#[test]
fn test1_1383() {
    assert_eq!(
        max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2),
        60
    );
    assert_eq!(
        max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3),
        68
    );
    assert_eq!(
        max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4),
        72
    );
}
