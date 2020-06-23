// https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum/
// Runtime: 56 ms
// Memory Usage: 5.1 MB
use std::collections::HashMap;
pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    let n = arr.len();
    let mut hm: HashMap<i32, i32> = HashMap::new();
    let mut dp = vec![i32::MAX; n];
    *hm.entry(0).or_default() = -1;
    let mut prev = 0;
    let mut res = i32::MAX;
    let mut min = i32::MAX;
    for i in 0..n {
        prev += arr[i];
        if let Some(&j) = hm.get(&(prev - target)) {
            if j > -1 && dp[j as usize] != i32::MAX {
                res = res.min(i as i32 - j + dp[j as usize]);
            }
            min = min.min(i as i32 - j);
        }
        dp[i] = min;
        *hm.entry(prev).or_default() = i as i32;
    }
    if res == i32::MAX {
        -1
    } else {
        res
    }
}
// dynamic_programming
#[test]
fn test1_1477() {
    assert_eq!(min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
    assert_eq!(min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
    assert_eq!(min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6), -1);
    assert_eq!(min_sum_of_lengths(vec![5, 5, 4, 4, 5], 3), -1);
    assert_eq!(min_sum_of_lengths(vec![3, 1, 1, 1, 5, 1, 2, 1], 3), 3);
}
