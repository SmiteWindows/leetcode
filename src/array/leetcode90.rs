// https://leetcode.com/problems/subsets-ii/
pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}
// array backtracking
#[test]
#[ignore]
fn test2_90() {
    assert_eq!(
        subsets_with_dup(vec![1, 2, 2]),
        vec![
            vec![2],
            vec![1],
            vec![1, 2, 2],
            vec![2, 2],
            vec![1, 2],
            vec![],
        ]
    );
}