// https://leetcode-cn.com/problems/find-pivot-index/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right: i32 = nums.iter().sum();
    for (i, num) in nums.iter().enumerate() {
        right -= num;
        if left == right {
            return i as i32;
        }
        left += num;
    }
    -1
}
// array
#[test]
fn test1_724() {
    assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(pivot_index(vec![1, 2, 3]), -1);
}
