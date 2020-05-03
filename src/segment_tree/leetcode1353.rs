// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/
pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// segment_tree greedy sort
#[test]
#[ignore]
fn test1_1353() {
    assert_eq!(max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]), 3);
    assert_eq!(
        max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]),
        4
    );
    assert_eq!(
        max_events(vec![
            vec![1, 4],
            vec![4, 4],
            vec![2, 2],
            vec![3, 4],
            vec![1, 1]
        ]),
        4
    );
    assert_eq!(max_events(vec![vec![1, 100000]]), 1);
    assert_eq!(
        max_events(vec![
            vec![1, 1],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![1, 5],
            vec![1, 6],
            vec![1, 7]
        ]),
        7
    );
}
