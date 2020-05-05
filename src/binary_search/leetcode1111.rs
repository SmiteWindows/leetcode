// https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings/
pub fn max_depth_after_split(seq: String) -> Vec<i32> {
    todo!()
}
// binary_search greedy
#[test]
#[ignore]
fn test1_1111() {
    assert_eq!(
        max_depth_after_split(String::from("(()())")),
        vec![0, 1, 1, 1, 1, 0]
    );
    assert_eq!(
        max_depth_after_split(String::from("()(())()")),
        vec![0, 0, 0, 1, 1, 0, 1, 1]
    );
}
