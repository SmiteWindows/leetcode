// https://leetcode-cn.com/problems/max-value-of-equation/
pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
    todo!()
}
// array sliding_window
#[test]
#[ignore]
fn test2_1499() {
    use leetcode_prelude::vec2;
    assert_eq!(
        find_max_value_of_equation(vec2![[1, 3], [2, 0], [5, 10], [6, -10]], 1),
        4
    );
    assert_eq!(
        find_max_value_of_equation(vec2![[0, 0], [3, 0], [9, 2]], 3),
        3
    );
}
