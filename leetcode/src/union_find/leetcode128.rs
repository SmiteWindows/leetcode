// https://leetcode-cn.com/problems/longest-consecutive-sequence/
use std::{collections::HashSet, iter::FromIterator};
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let hs: HashSet<i32> = HashSet::from_iter(nums);
    let mut res = 0;
    for &x in &hs {
        if hs.contains(&(x - 1)) {
            continue;
        }
        let mut i = 1;
        while hs.contains(&(x + i)) {
            i += 1;
            res = res.max(i);
        }
    }
    res
}
// array union_find
#[test]
#[ignore]
fn test1_128() {
    assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
}
