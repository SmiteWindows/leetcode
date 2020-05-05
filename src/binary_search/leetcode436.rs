// https://leetcode.com/problems/find-right-interval/
pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// binary_search
#[test]
#[ignore]
fn test1_436() {
    assert_eq!(find_right_interval(vec![vec![1, 2]]), vec![-1]);
    assert_eq!(
        find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
        vec![-1, 0, 1]
    );
    assert_eq!(
        find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
        vec![-1, 2, -1]
    );
}
