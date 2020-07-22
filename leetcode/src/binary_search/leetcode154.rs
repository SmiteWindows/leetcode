// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;
    while l < r {
        let m = l + (r - l) / 2;
        match nums[m].cmp(&nums[r]) {
            Less => r = m,
            Greater => l = m + 1,
            Equal => r -= 1,
        }
    }
    nums[l]
}
// binary_search array
#[test]
fn test1_154() {
    assert_eq!(find_min(vec![1, 3, 5]), 1);
    assert_eq!(find_min(vec![2, 2, 2, 0, 1]), 0);
}
