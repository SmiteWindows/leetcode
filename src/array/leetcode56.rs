// https://leetcode.com/problems/merge-intervals/
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}
// array sort
#[test]
#[ignore]
fn test2_56() {
    assert_eq!(
        merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    );
    assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
}
