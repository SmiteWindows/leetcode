// https://leetcode.com/problems/jump-game-v/
// Runtime: 92 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
    let n = arr.len();
    let d = d as usize;
    let mut memo = HashMap::new();
    let mut res = 0;
    for i in 0..n {
        res = res.max(dp(i, &mut memo, &arr, d, n));
    }
    res
}

fn dp(start: usize, memo: &mut HashMap<usize, i32>, arr: &[i32], d: usize, n: usize) -> i32 {
    if let Some(&res) = memo.get(&start) {
        return res;
    }
    let mut max = 0;
    let mut i = 1;
    while i <= d && start >= i && arr[start] > arr[start - i] {
        max = max.max(dp(start - i, memo, arr, d, n));
        i += 1;
    }
    let mut i = 1;
    while i <= d && start + i < n && arr[start] > arr[start + i] {
        max = max.max(dp(start + i, memo, arr, d, n));
        i += 1;
    }
    let res = 1 + max;
    memo.insert(start, res);
    res
}
// dynamic_programming
#[test]
fn test1_1340() {
    assert_eq!(max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2), 4);
    assert_eq!(max_jumps(vec![3, 3, 3, 3, 3], 3), 1);
    assert_eq!(max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
    assert_eq!(max_jumps(vec![7, 1, 7, 1, 7, 1], 2), 2);
    assert_eq!(max_jumps(vec![66], 1), 1);
}
