// https://leetcode-cn.com/problems/minimum-add-to-make-parentheses-valid/
pub fn min_add_to_make_valid(s: String) -> i32 {
    todo!()
}
// stack greedy
#[test]
#[ignore]
fn test2_921() {
    assert_eq!(min_add_to_make_valid("())".to_string()), 1);
    assert_eq!(min_add_to_make_valid("(((".to_string()), 3);
    assert_eq!(min_add_to_make_valid("()".to_string()), 0);
    assert_eq!(min_add_to_make_valid("()))((".to_string()), 4);
}
