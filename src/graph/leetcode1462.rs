// https://leetcode.com/problems/course-schedule-iv/
pub fn check_if_prerequisite(
    n: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    todo!()
}
// graph
#[test]
#[ignore]
fn test1_1462() {
    assert_eq!(
        check_if_prerequisite(2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]),
        vec![false, true]
    );
    assert_eq!(
        check_if_prerequisite(2, vec![vec![]], vec![vec![1, 0], vec![0, 1]]),
        vec![false, false]
    );
    assert_eq!(
        check_if_prerequisite(
            3,
            vec![vec![1, 2], vec![1, 0], vec![2, 0]],
            vec![vec![1, 0], vec![1, 2]]
        ),
        vec![true, true]
    );
    assert_eq!(
        check_if_prerequisite(
            3,
            vec![vec![1, 0], vec![2, 0]],
            vec![vec![0, 1], vec![2, 0]]
        ),
        vec![false, true]
    );
    assert_eq!(
        check_if_prerequisite(
            5,
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]],
            vec![vec![0, 4], vec![4, 0], vec![1, 3], vec![3, 0]]
        ),
        vec![true, false, true, false]
    );
}
