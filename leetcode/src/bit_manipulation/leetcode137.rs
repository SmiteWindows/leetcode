// https://leetcode.com/problems/single-number-ii/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut seen_once = 0;
    let mut seen_twice = 0;
    for i in nums {
        seen_once = !seen_twice & (seen_once ^ i);
        seen_twice = !seen_once & (seen_twice ^ i);
    }
    seen_once
}
// bit_manipulation
#[test]
fn test1_137() {
    assert_eq!(single_number(vec![2, 2, 3, 2]), 3);
    assert_eq!(single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
}
