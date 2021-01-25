// https://leetcode-cn.com/problems/find-peak-element/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut h = nums.len() - 1;
    while l < h {
        let m1 = l + (h - l) / 2;
        let m2 = m1 + 1;
        if nums[m1] < nums[m2] {
            l = m2;
        } else {
            h = m1;
        }
    }
    l as i32
}
// binary_search array
#[test]
fn test1_162() {
    assert_eq!(find_peak_element(vec![1, 2, 3, 1]), 2);
    assert_eq!(find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
}
