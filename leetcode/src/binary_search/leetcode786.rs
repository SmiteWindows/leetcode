// https://leetcode-cn.com/problems/k-th-smallest-prime-fraction/
// Runtime: 292 ms
// Memory Usage: 2 MB
use std::{cmp::Ordering, collections::BinaryHeap};
pub fn kth_smallest_prime_fraction(a: Vec<i32>, k: i32) -> Vec<i32> {
    let mut queue = BinaryHeap::new();
    let n = a.len();
    let k = k as usize;
    for i in 0..n {
        queue.push(Fraction(a[i], a[n - 1], i, n - 1));
    }
    for _ in 0..k - 1 {
        let f = queue.pop().unwrap();
        if f.3 - 1 > f.2 {
            queue.push(Fraction(a[f.2], a[f.3 - 1], f.2, f.3 - 1));
        }
    }
    let f = queue.pop().unwrap();
    vec![f.0, f.1]
}

struct Fraction(i32, i32, usize, usize);

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.0 * other.1 == self.1 * other.0
    }
}

impl Eq for Fraction {}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.1 * other.0).cmp(&(self.0 * other.1))
    }
}
// binary_search heap
#[test]
fn test1_786() {
    assert_eq!(kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3), vec![2, 5]);
    assert_eq!(kth_smallest_prime_fraction(vec![1, 7], 1), vec![1, 7]);
}
