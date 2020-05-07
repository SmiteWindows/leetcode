// https://leetcode.com/problems/increasing-subsequences/
pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_491() {
    assert_eq!(
        find_subsequences(vec![4, 6, 7, 7]),
        vec![
            vec![4, 6],
            vec![4, 7],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7],
            vec![4, 7, 7]
        ]
    );
}
