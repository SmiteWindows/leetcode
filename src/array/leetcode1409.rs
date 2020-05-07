// https://leetcode.com/problems/queries-on-a-permutation-with-key/
pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1409() {
    assert_eq!(process_queries(vec![3, 1, 2, 1], 5), vec![2, 1, 2, 1]);
    assert_eq!(process_queries(vec![4, 1, 2, 2], 4), vec![3, 1, 2, 0]);
    assert_eq!(process_queries(vec![7, 5, 5, 8, 3], 8), vec![6, 5, 0, 7, 5]);
}
