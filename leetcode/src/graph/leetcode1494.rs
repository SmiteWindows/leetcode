// https://leetcode-cn.com/problems/parallel-courses-ii/
pub fn min_number_of_semesters(n: i32, dependencies: Vec<Vec<i32>>, k: i32) -> i32 {
    todo!()
}
// graph
#[test]
#[ignore]
fn test1_1494() {
    use leetcode_prelude::vec2;
    assert_eq!(
        min_number_of_semesters(4, vec2![[2, 1], [3, 1], [1, 4]], 2),
        3
    );
    assert_eq!(
        min_number_of_semesters(5, vec2![[2, 1], [3, 1], [4, 1], [1, 5]], 2),
        4
    );
    assert_eq!(min_number_of_semesters(11, vec![], 2), 6);
}
