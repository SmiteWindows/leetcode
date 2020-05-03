// https://leetcode.com/problems/4sum/
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    todo!()
}
// array two_pointers hash_table
#[test]
#[ignore]
fn test1_18() {
    assert_eq!(
        four_sum(vec![1, 0, -1, 0, -2, 2], 0),
        vec![vec![-1, 0, 0, 1], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2]]
    );
}
