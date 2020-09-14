// https://leetcode-cn.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn max_depth_after_split(seq: String) -> Vec<i32> {
    let n = seq.len();
    let mut level = 0;
    let mut res = vec![0; n];
    for (i, c) in seq.char_indices() {
        if c == '(' {
            res[i] = level % 2;
            level += 1;
        } else {
            level -= 1;
            res[i] = level % 2;
        }
    }
    res
}
// binary_search greedy
#[test]
fn test1_1111() {
    assert_eq!(
        max_depth_after_split("(()())".to_string()),
        vec![0, 1, 1, 1, 1, 0]
    );
    assert_eq!(
        max_depth_after_split("()(())()".to_string()),
        vec![0, 0, 0, 1, 1, 0, 0, 0]
    );
}
