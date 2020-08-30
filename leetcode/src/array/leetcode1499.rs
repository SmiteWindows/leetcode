// https://leetcode-cn.com/problems/max-value-of-equation/
pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
    todo!()
}
// array sliding_window
#[test]
#[ignore]
fn test2_1499() {
    assert_eq!(
        find_max_value_of_equation(vec![vec![1, 3], vec![2, 0], vec![5, 10], vec![6, -10]], 1),
        4
    );
    assert_eq!(
        find_max_value_of_equation(vec![vec![0, 0], vec![3, 0], vec![9, 2]], 3),
        3
    );
}
