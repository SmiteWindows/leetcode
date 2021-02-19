// https://leetcode-cn.com/problems/maximum-number-of-non-overlapping-subarrays-with-sum-equals-target/
// Runtime: 40 ms
// Memory Usage: 6.9 MB
use std::collections::HashMap;
pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
    let mut sum = 0;
    let mut hm: HashMap<i32, usize> = HashMap::new();
    let mut last = 0;
    hm.insert(0, 0);
    let mut res = 0;
    for (i, &num) in nums.iter().enumerate() {
        let x = num;
        sum += x;
        if let Some(&j) = hm.get(&(sum - target)) {
            if j >= last {
                res += 1;
                last = i + 1;
            }
        }
        *hm.entry(sum).or_default() = i + 1;
    }
    res
}
// dynamic_programming
#[test]
fn test1_1546() {
    assert_eq!(max_non_overlapping(vec![1, 1, 1, 1, 1], 2), 2);
    assert_eq!(max_non_overlapping(vec![-1, 3, 5, 1, 4, 2, -9], 6), 2);
    assert_eq!(max_non_overlapping(vec![-2, 6, 6, 3, 5, 4, 1, 2, 8], 10), 3);
    assert_eq!(max_non_overlapping(vec![0, 0, 0], 0), 3);
}
