// https://leetcode.com/problems/running-sum-of-1d-array/
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1480() {
    assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(running_sum(vec![1, 1, 1, 1, 1]), vec![1, 2, 3, 4, 5]);
    assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
}
