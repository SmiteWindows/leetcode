// https://leetcode.com/problems/last-moment-before-all-ants-fall-out-of-a-plank/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
    left.into_iter()
        .chain(right.into_iter().map(|x| n - x))
        .max()
        .unwrap()
}
// array brainteaser
#[test]
fn test2_1503() {
    assert_eq!(get_last_moment(4, vec![4, 3], vec![0, 1]), 4);
    assert_eq!(get_last_moment(7, vec![], vec![0, 1, 2, 3, 4, 5, 6, 7]), 7);
    assert_eq!(get_last_moment(7, vec![0, 1, 2, 3, 4, 5, 6, 7], vec![]), 7);
    assert_eq!(get_last_moment(9, vec![5], vec![4]), 5);
    assert_eq!(get_last_moment(6, vec![6], vec![0]), 6);
}
