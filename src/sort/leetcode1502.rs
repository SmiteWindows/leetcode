// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
    let mut arr = arr;
    arr.sort_unstable();
    let diff = arr.windows(2).map(|v| v[1] - v[0]).collect::<Vec<i32>>();
    diff.iter().min() == diff.iter().max()
}
// array sort
#[test]
fn test2_1502() {
    assert_eq!(can_make_arithmetic_progression(vec![3, 5, 1]), true);
    assert_eq!(can_make_arithmetic_progression(vec![1, 2, 4]), false);
}
