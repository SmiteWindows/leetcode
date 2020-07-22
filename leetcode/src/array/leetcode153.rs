// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;
    while l < r {
        if nums[l] < nums[r] {
            return nums[l];
        }
        let m = l + (r - l) / 2;
        if nums[l] <= nums[m] {
            l = m + 1;
        } else {
            r = m;
        }
    }
    nums[l]
}
// binary_search array
#[test]
fn test2_153() {
    assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
}
