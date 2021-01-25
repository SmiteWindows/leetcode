// https://leetcode-cn.com/problems/arithmetic-slices-ii-subsequence/
// Runtime: 116 ms
// Memory Usage: 7.4 MB
use std::collections::HashMap;
pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
    let n = a.len();
    let mut ans = 0i64;
    let mut cnt = vec![HashMap::<i32, i32>::new(); n];
    for i in 0..n {
        for j in 0..i {
            let delta = a[i] as i64 - a[j] as i64;
            if delta < i32::MIN as i64 || delta > i32::MAX as i64 {
                continue;
            }
            let diff = delta as i32;
            let sum = *cnt[j].get(&diff).or(Some(&0)).unwrap();
            let origin = *cnt[i].get(&diff).or(Some(&0)).unwrap();
            *cnt[i].entry(diff).or_default() = origin + sum + 1;
            ans += sum as i64;
        }
    }
    ans as i32
}
// dynamic_programming
#[test]
fn test1_446() {
    assert_eq!(number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]), 7);
}
