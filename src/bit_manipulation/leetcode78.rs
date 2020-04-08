// https://leetcode.com/problems/subsets/
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}
//  array backtracking bit_manipulation
#[test]
fn test_3_78() {
    let nums = vec![1];
    let res = vec![
        vec![],
        vec![1],
    ];
    assert_eq!(subsets(nums), res);
    let nums = vec![1, 2, 3];
    let res = vec![
        vec![],
        vec![1],
        vec![2],
        vec![1, 2],
        vec![3],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
    ];
    assert_eq!(subsets(nums), res);
}
