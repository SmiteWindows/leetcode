// https://leetcode.com/problems/reduce-array-size-to-the-half/
pub fn min_set_size(arr: Vec<i32>) -> i32 {
    todo!()
}
// greedy array
#[test]
#[ignore]
fn test1_1338() {
    assert_eq!(min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]), 2);
    assert_eq!(min_set_size(vec![7, 7, 7, 7, 7, 7]), 1);
    assert_eq!(min_set_size(vec![1, 9]), 1);
    assert_eq!(min_set_size(vec![1000, 1000, 3, 7]), 1);
    assert_eq!(min_set_size(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 5);
}
