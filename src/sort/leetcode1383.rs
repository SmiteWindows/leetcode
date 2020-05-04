// https://leetcode.com/problems/maximum-performance-of-a-team/
pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
    todo!()
}
// sort greedy
#[test]
#[ignore]
fn test1_1356() {
    assert_eq!(
        max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2),
        60
    );
    assert_eq!(
        max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3),
        68
    );
    assert_eq!(
        max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4),
        72
    );
}
