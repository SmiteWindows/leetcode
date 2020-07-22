// https://leetcode.com/problems/set-intersection-size-at-least-two/
pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_757() {
    assert_eq!(
        intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]]),
        3
    );
    assert_eq!(
        intersection_size_two(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]]),
        5
    );
}
