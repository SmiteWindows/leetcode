// https://leetcode.com/problems/reduce-array-size-to-the-half/
// Runtime: 24 ms
// Memory Usage: 7.1 MB
use std::collections::{BinaryHeap, HashMap};
pub fn min_set_size(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut hm: HashMap<i32, usize> = HashMap::new();
    for x in arr {
        *hm.entry(x).or_default() += 1;
    }
    let mut pq = BinaryHeap::new();
    for (_, v) in hm {
        pq.push(v);
    }
    let mut sum = 0;
    let mut res = 0;
    while sum * 2 < n {
        sum += pq.pop().unwrap();
        res += 1;
    }
    res
}
// greedy array
#[test]
fn test1_1338() {
    assert_eq!(min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]), 2);
    assert_eq!(min_set_size(vec![7, 7, 7, 7, 7, 7]), 1);
    assert_eq!(min_set_size(vec![1, 9]), 1);
    assert_eq!(min_set_size(vec![1000, 1000, 3, 7]), 1);
    assert_eq!(min_set_size(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5);
}
