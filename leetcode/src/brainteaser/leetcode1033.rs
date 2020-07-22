// https://leetcode.com/problems/moving-stones-until-consecutive/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
    let mut a = vec![a, b, c];
    a.sort_unstable();
    let min = if a[0] + 1 == a[1] && a[1] + 1 == a[2] {
        0
    } else if a[0] + 2 >= a[1] || a[1] + 2 >= a[2] {
        1
    } else {
        2
    };
    let max = a[1] - a[0] - 1 + a[2] - a[1] - 1;
    vec![min, max]
}
// brainteaser
#[test]
fn test1_1033() {
    assert_eq!(num_moves_stones(1, 2, 5), vec![1, 2]);
    assert_eq!(num_moves_stones(4, 3, 2), vec![0, 0]);
    assert_eq!(num_moves_stones(3, 5, 1), vec![1, 2]);
}
