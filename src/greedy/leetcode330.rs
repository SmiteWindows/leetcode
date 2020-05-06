// https://leetcode.com/problems/patching-array/
pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_330() {
    assert_eq!(min_patches(vec![1, 3], 6), 1);
    assert_eq!(min_patches(vec![1, 5, 10], 20), 2);
    assert_eq!(min_patches(vec![1, 2, 2], 5), 0);
}
