// https://leetcode.com/problems/single-element-in-a-sorted-array/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        let mut m = l + (r - l) / 2;
        if m % 2 == 1 {
            m -= 1;
        }
        if nums[m + 1] == nums[m] {
            l = m + 2;
        } else {
            r = m;
        }
    }
    nums[l]
}
#[test]
fn test540() {
    assert_eq!(single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
    assert_eq!(single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]), 10);
}
