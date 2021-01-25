// https://leetcode-cn.com/problems/longest-subarray-of-1s-after-deleting-one-element/
// Runtime: 4 ms
// Memory Usage: 3 MB
pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let mut prev_left = 0;
    for i in 0..n {
        left[i] = prev_left;
        if nums[i] == 0 {
            prev_left = 0;
        } else {
            prev_left += 1;
        }
    }
    let mut prev_right = 0;
    for i in (0..n).rev() {
        right[i] = prev_right;
        if nums[i] == 0 {
            prev_right = 0;
        } else {
            prev_right += 1;
        }
    }
    let mut res = 0;
    for i in 0..n {
        res = res.max(left[i] + right[i]);
    }
    res
}
// array
#[test]
fn test1_1493() {
    assert_eq!(longest_subarray(vec![1, 1, 0, 1]), 3);
    assert_eq!(longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
    assert_eq!(longest_subarray(vec![1, 1, 1]), 2);
    assert_eq!(longest_subarray(vec![1, 1, 0, 0, 1, 1, 1, 0, 1]), 4);
    assert_eq!(longest_subarray(vec![0, 0, 0]), 0);
}
