// https://leetcode-cn.com/problems/search-in-rotated-sorted-array-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn search(nums: Vec<i32>, target: i32) -> bool {
    let n = nums.len();
    if n == 0 {
        return false;
    }
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] == target {
            return true;
        }
        match nums[m].cmp(&nums[r]) {
            Equal => {
                r -= 1;
            }
            Less => {
                if nums[m] < target && nums[r] >= target {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            Greater => {
                if nums[m] > target && nums[l] <= target {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
        }
    }
    nums[l] == target
}
// binary_search array
#[test]
fn test1_81() {
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
}
