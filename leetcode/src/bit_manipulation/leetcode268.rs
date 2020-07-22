// https://leetcode.com/problems/missing-number/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut n = nums.len() as i32;
    for (i, j) in nums.iter().enumerate() {
        let i = i as i32;
        n ^= i ^ j;
    }
    n
}
// bit_manipulation array math
#[test]
fn test1_268() {
    assert_eq!(missing_number(vec![3, 0, 1]), 2);
    assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
}
