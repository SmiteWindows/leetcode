// https://leetcode-cn.com/problems/least-number-of-unique-integers-after-k-removals/
// Runtime: 24 ms
// Memory Usage: 7.1 MB
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    let mut k = k as usize;
    let mut count: HashMap<i32, usize> = HashMap::new();
    for x in arr {
        *count.entry(x).or_default() += 1;
    }
    let mut queue = BinaryHeap::new();
    for (_, v) in count {
        queue.push(Reverse(v));
    }
    while let Some(&min) = queue.peek() {
        if min.0 <= k {
            k -= min.0;
            queue.pop();
        } else {
            break;
        }
    }
    queue.len() as i32
}
// array sort
#[test]
fn test1_1481() {
    assert_eq!(find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
    assert_eq!(
        find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3),
        2
    );
}
