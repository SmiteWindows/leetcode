// https://leetcode-cn.com/problems/missing-number/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let expected_sum = n * (n + 1) / 2;
    let mut actual_sum = 0;
    for i in nums {
        actual_sum += i;
    }
    expected_sum - actual_sum
}
// bit_manipulation array math
#[test]
fn test3_268() {
    assert_eq!(missing_number(vec![3, 0, 1]), 2);
    assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
}
