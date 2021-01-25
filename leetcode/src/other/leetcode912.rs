// https://leetcode-cn.com/problems/sort-an-array/
// Runtime: 8 ms
// Memory Usage: 2.6 MB
pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    nums
}
#[test]
fn test912() {
    assert_eq!(sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    assert_eq!(sort_array(vec![5, 1, 1, 2, 0, 0]), vec![0, 0, 1, 1, 2, 5]);
}
