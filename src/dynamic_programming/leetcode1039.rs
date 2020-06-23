// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/
// Runtime: 8 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn min_score_triangulation(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut memo = HashMap::new();
    dp(0, n - 1, &mut memo, &a, n)
}

fn dp(i: usize, j: usize, memo: &mut HashMap<(usize, usize), i32>, a: &[i32], n: usize) -> i32 {
    if let Some(&res) = memo.get(&(i, j)) {
        return res;
    }
    let mut res = i32::MAX;
    for k in i + 1..j {
        let left = dp(i, k, memo, a, n);
        let right = dp(k, j, memo, a, n);
        let mid = a[i] * a[j] * a[k];
        res = res.min(left + right + mid);
    }
    if res == i32::MAX {
        res = 0;
    }
    memo.insert((i, j), res);
    res
}
// dynamic_programming
#[test]
fn test1_1039() {
    assert_eq!(min_score_triangulation(vec![1, 2, 3]), 6);
    assert_eq!(min_score_triangulation(vec![3, 7, 4, 5]), 144);
    assert_eq!(min_score_triangulation(vec![1, 3, 1, 4, 1, 5]), 13);
}
