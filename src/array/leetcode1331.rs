// https://leetcode.com/problems/rank-transform-of-an-array/
pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1331() {
    assert_eq!(array_rank_transform(vec![40, 10, 20, 30]), vec![4, 1, 2, 3]);
    assert_eq!(array_rank_transform(vec![100, 100, 100]), vec![1, 1, 1]);
    assert_eq!(
        array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
        vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
    );
}
