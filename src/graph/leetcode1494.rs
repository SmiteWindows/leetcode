// https://leetcode.com/problems/parallel-courses-ii/
pub fn min_number_of_semesters(n: i32, dependencies: Vec<Vec<i32>>, k: i32) -> i32 {
    todo!()
}
// graph
#[test]
#[ignore]
fn test1_1494() {
    assert_eq!(
        min_number_of_semesters(4, vec![vec![2, 1], vec![3, 1], vec![1, 4]], 2),
        3
    );
    assert_eq!(
        min_number_of_semesters(5, vec![vec![2, 1], vec![3, 1], vec![4, 1], vec![1, 5]], 2),
        4
    );
    assert_eq!(min_number_of_semesters(11, vec![], 2), 6);
}
