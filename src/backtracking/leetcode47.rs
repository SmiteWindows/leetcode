// https://leetcode.com/problems/permutations-ii/
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}
// backtracking
#[test]
#[ignore]
fn test1_47() {
    assert_eq!(
        permute_unique(vec![1, 1, 2]),
        vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
    );
}
