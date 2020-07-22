// https://leetcode.com/problems/pizza-with-3n-slices/
pub fn max_size_slices(slices: Vec<i32>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1388() {
    assert_eq!(max_size_slices(vec![1, 2, 3, 4, 5, 6]), 10);
    assert_eq!(max_size_slices(vec![8, 9, 8, 6, 1, 1]), 16);
    assert_eq!(max_size_slices(vec![4, 1, 2, 5, 8, 3, 1, 9, 7]), 21);
    assert_eq!(max_size_slices(vec![3, 1, 2]), 3);
}
