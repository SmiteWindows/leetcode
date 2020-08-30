// https://leetcode-cn.com/problems/minimum-moves-to-equal-array-elements/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
pub fn min_moves(nums: Vec<i32>) -> i32 {
    let min = nums.iter().min().unwrap();
    let mut sum = 0;
    for &x in &nums {
        sum += x - min;
    }
    sum
}
// math
#[test]
fn test1_453() {
    assert_eq!(min_moves(vec![1, 2, 3]), 3);
}
