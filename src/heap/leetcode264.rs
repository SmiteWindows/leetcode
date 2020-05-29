// https://leetcode.com/problems/ugly-number-ii/
// Runtime: 36 ms
// Memory Usage: 2.1 MB
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};
pub fn nth_ugly_number(n: i32) -> i32 {
    let n = n as usize;
    let mut nums = vec![0; n];
    let mut seen = HashSet::new();
    let mut heap = BinaryHeap::new();
    seen.insert(1_i64);
    heap.push(Reverse(1_i64));
    let mut new;
    let primes = vec![2, 3, 5];
    for num in nums.iter_mut().take(n) {
        if let Some(Reverse(curr)) = heap.pop() {
            *num = curr;
            for &j in &primes {
                new = curr * j;
                if !seen.contains(&new) {
                    seen.insert(new);
                    heap.push(Reverse(new));
                }
            }
        };
    }
    nums[n - 1] as i32
}
// math heap dynamic_programming
#[test]
fn test3_264() {
    assert_eq!(nth_ugly_number(10), 12);
}
