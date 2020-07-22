// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/
// Runtime: 96 ms
// Memory Usage: 2.2 MB
use std::collections::BTreeSet;
pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut prefix = vec![vec![0; m + 1]; n];
    for i in 0..n {
        for j in 0..m {
            prefix[i][j + 1] = prefix[i][j] + matrix[i][j];
        }
    }
    let mut res = i32::MIN;
    for start in 0..m {
        for end in start + 1..=m {
            let mut bts = BTreeSet::new();
            bts.insert(0);
            let mut sum = 0;
            for p in prefix.iter().take(n) {
                sum += p[end] - p[start];
                if let Some(prev) = bts.range(sum - k..).take(1).next() {
                    res = res.max(sum - prev);
                }
                bts.insert(sum);
            }
        }
    }
    res
}
// binary_search dynamic_programming queue
#[test]
fn test3_363() {
    assert_eq!(max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2), 2);
}
