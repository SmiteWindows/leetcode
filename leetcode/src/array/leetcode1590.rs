// https://leetcode.com/problems/make-sum-divisible-by-p/
// Runtime: 32 ms
// Memory Usage: 6.8 MB
use std::collections::HashMap;
pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let n = nums.len();
    let mut total = 0;
    for i in 0..n {
        total += nums[i];
        total %= p;
    }
    if total == 0 {
        0
    } else {
        let mut res = usize::MAX;
        let mut index: HashMap<i32, usize> = HashMap::new();
        index.insert(0, 0);
        let mut cur = 0;
        for i in 0..n {
            cur += nums[i];
            cur %= p;
            let comp = (cur + p - total) % p;

            if let Some(j) = index.get(&comp) {
                res = res.min(i + 1 - j);
            }
            index.insert(cur, i + 1);
        }
        if res < n {
            res as i32
        } else {
            -1
        }
    }
}
// array binary_search
#[test]
fn test1_1590() {
    assert_eq!(min_subarray(vec![3, 1, 4, 2], 6), 1);
    assert_eq!(min_subarray(vec![6, 3, 5, 2], 9), 2);
    assert_eq!(min_subarray(vec![1, 2, 3], 3), 0);
    assert_eq!(min_subarray(vec![1, 2, 3], 7), -1);
    assert_eq!(min_subarray(vec![1000000000, 1000000000, 1000000000], 3), 0);
}
