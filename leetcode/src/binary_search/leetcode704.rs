// https://leetcode.com/problems/binary-search/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(a) => a as i32,
        Err(_) => -1,
    }
}
// binary_search
#[test]
fn test1_704() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
}
