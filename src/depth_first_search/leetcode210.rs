// https://leetcode.com/problems/course-schedule-ii/
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// depth_first_search topological_sort graph breadth_first_search
#[test]
#[ignore]
fn test3_210() {
    assert_eq!(find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    assert_eq!(
        find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
        vec![0, 1, 2, 3]
    );
    assert_eq!(
        find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
        vec![0, 2, 1, 3]
    );
}
