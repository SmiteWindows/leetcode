// https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/
pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    todo!()
}
// binary_search
#[test]
#[ignore]
fn test1_1292() {
    assert_eq!(smallest_divisor(vec![1, 2, 5, 9], 6), 5);
    assert_eq!(smallest_divisor(vec![2, 3, 5, 7, 11], 11), 3);
    assert_eq!(smallest_divisor(vec![19], 5), 4);
}
