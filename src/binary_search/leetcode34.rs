// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
// Runtime: 0 ms
// Memory Usage: 2.4 MB
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let n = nums.len();
    match nums.binary_search(&target) {
        Ok(i) => {
            let mut l = i;
            let mut r = i;
            while l > 0 && nums[l - 1] == target {
                l -= 1;
            }
            while r + 1 < n && nums[r + 1] == target {
                r -= 1;
            }
            vec![l as i32, r as i32]
        }
        Err(_) => vec![-1, -1],
    }
}
// binary_search array
#[test]
fn test1_34() {
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
}
