// https://leetcode-cn.com/problems/get-the-maximum-score/
// Runtime: 84 ms
// Memory Usage: 5.3 MB
use std::collections::{BTreeSet, HashSet};
const MOD: i64 = 1_000_000_007;
pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let set1: HashSet<i32> = nums1.iter().copied().collect();
    let set2: HashSet<i32> = nums2.iter().copied().collect();
    let all: BTreeSet<i32> = nums1.iter().copied().chain(nums2.iter().copied()).collect();
    let mut prev1 = 0;
    let mut prev2 = 0;
    let mut res = 0;
    for x in all {
        if set1.contains(&x) {
            prev1 += x as i64;
        }
        if set2.contains(&x) {
            prev2 += x as i64;
        }
        if set1.contains(&x) && set2.contains(&x) {
            let max = prev1.max(prev2);
            prev1 = max;
            prev2 = max;
        }
        res = res.max(prev1);
        res = res.max(prev2);
    }
    res %= MOD;
    res as i32
}
// dynamic_programming
#[test]
fn test1_1537() {
    assert_eq!(max_sum(vec![2, 4, 5, 8, 10], vec![4, 6, 8, 9]), 30);
    assert_eq!(max_sum(vec![1, 3, 5, 7, 9], vec![3, 5, 100]), 109);
    assert_eq!(max_sum(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]), 40);
    assert_eq!(
        max_sum(vec![1, 4, 5, 8, 9, 11, 19], vec![2, 3, 4, 11, 12]),
        61
    );
}
