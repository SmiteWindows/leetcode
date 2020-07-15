// https://leetcode.com/problems/frog-jump/
// Runtime: 8 ms
// Memory Usage: 3.5 MB
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};
pub fn can_cross(stones: Vec<i32>) -> bool {
    let n = stones.len();
    let end = stones[n - 1];
    let mut memo = HashMap::new();
    let stones = HashSet::from_iter(stones);
    dp(0, 1, &mut memo, &stones, end)
}

fn dp(
    start: i32,
    k: i32,
    memo: &mut HashMap<(i32, i32), bool>,
    stones: &HashSet<i32>,
    end: i32,
) -> bool {
    if let Some(&res) = memo.get(&(start, k)) {
        return res;
    }
    let res = if start + k == end {
        true
    } else {
        if stones.contains(&(start + k)) {
            if k - 1 > 0 {
                dp(start + k, k + 1, memo, stones, end)
                    || dp(start + k, k, memo, stones, end)
                    || dp(start + k, k - 1, memo, stones, end)
            } else {
                dp(start + k, k + 1, memo, stones, end) || dp(start + k, k, memo, stones, end)
            }
        } else {
            false
        }
    };
    memo.insert((start, k), res);
    res
}
// dynamic_programming
#[test]
fn test1_403() {
    assert_eq!(can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]), true);
    assert_eq!(can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]), false);
}
