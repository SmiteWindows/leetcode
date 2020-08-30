// https://leetcode-cn.com/problems/search-in-rotated-sorted-array/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut l: i32 = 0;
    let mut r: i32 = n as i32 - 1;
    while l <= r {
        let m = (l + r) / 2;

        if target == nums[m as usize] {
            return m as i32;
        } else if nums[l as usize] <= nums[m as usize] {
            if nums[l as usize] <= target && target <= nums[m as usize] {
                r = m - 1;
            } else {
                l = m + 1;
            }
        } else if nums[m as usize] <= target && target <= nums[r as usize] {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    -1
}
// binary_search array
#[test]
fn test2_33() {
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
}
