// https://leetcode-cn.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/
// Runtime: 8 ms
// Memory Usage: 4.2 MB
use std::collections::VecDeque;
pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut max_queue = VecDeque::new();
    let mut min_queue = VecDeque::new();
    let n = nums.len();
    let mut res = 0;
    let mut l = 0;
    for r in 0..n {
        while let Some(&last) = max_queue.back() {
            if last < nums[r] {
                max_queue.pop_back();
            } else {
                break;
            }
        }
        while let Some(&last) = min_queue.back() {
            if last > nums[r] {
                min_queue.pop_back();
            } else {
                break;
            }
        }
        max_queue.push_back(nums[r]);
        min_queue.push_back(nums[r]);
        if max_queue.front().unwrap() - min_queue.front().unwrap() > limit {
            if *max_queue.front().unwrap() == nums[l] {
                max_queue.pop_front();
            }
            if *min_queue.front().unwrap() == nums[l] {
                min_queue.pop_front();
            }
            l += 1;
        }
        res = res.max(r - l + 1);
    }
    res as i32
}
// array sliding_window
#[test]
fn test2_1438() {
    assert_eq!(longest_subarray(vec![8, 2, 4, 7], 4), 2);
    assert_eq!(longest_subarray(vec![10, 1, 2, 4, 7, 2], 5), 4);
    assert_eq!(longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0), 3);
}
