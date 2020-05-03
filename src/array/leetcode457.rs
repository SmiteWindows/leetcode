// https://leetcode.com/problems/circular-array-loop/
pub fn circular_array_loop(nums: Vec<i32>) -> bool {
    todo!()
}
// array two_pointers
#[test]
#[ignore]
fn test2_457() {
    assert_eq!(circular_array_loop(vec![2, -1, 1, 2, 2]), true);
    assert_eq!(circular_array_loop(vec![-1, 2]), false);
    assert_eq!(circular_array_loop(vec![-2, 1, -1, -2, -2]), false);
}
