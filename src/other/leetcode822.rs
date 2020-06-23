// https://leetcode.com/problems/card-flipping-game/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
    let n = fronts.len();
    let mut hs = HashSet::new();
    let mut res = i32::MAX;
    for i in 0..n {
        if fronts[i] == backs[i] {
            hs.insert(fronts[i]);
        }
    }
    for i in 0..n {
        if !hs.contains(&fronts[i]) {
            res = res.min(fronts[i]);
        }
        if !hs.contains(&backs[i]) {
            res = res.min(backs[i]);
        }
    }
    if res == i32::MAX {
        0
    } else {
        res
    }
}
#[test]
fn test822() {
    assert_eq!(flipgame(vec![1, 2, 4, 4, 7], vec![1, 3, 4, 1, 3]), 2);
}
