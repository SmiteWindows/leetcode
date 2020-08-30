// https://leetcode-cn.com/problems/super-ugly-number/
// Runtime: 60 ms
// Memory Usage: 7.1 MB
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};
pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
    let mut n = n;
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(Reverse(1));
    while n > 1 {
        let min = queue.pop().unwrap().0;
        for &x in &primes {
            if let Some(y) = x.checked_mul(min) {
                if visited.insert(y) {
                    queue.push(Reverse(y));
                }
            }
        }
        n -= 1;
    }
    queue.pop().unwrap().0
}
// math heap
#[test]
fn test2_313() {
    assert_eq!(nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
}
