// https://leetcode.com/problems/make-array-strictly-increasing/
pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1187() {
    assert_eq!(
        make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]),
        1
    );
    assert_eq!(make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1]), 2);
    assert_eq!(
        make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3]),
        -1
    );
}
