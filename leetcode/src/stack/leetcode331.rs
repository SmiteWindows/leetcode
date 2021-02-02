// https://leetcode-cn.com/problems/verify-preorder-serialization-of-a-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn is_valid_serialization(preorder: String) -> bool {
    let mut stack = Vec::new();
    for tok in preorder.split(',') {
        if tok == "#" {
            while let Some('#') = stack.last() {
                stack.pop();
                if stack.pop().is_none() {
                    return false;
                };
            }
            stack.push('#');
        } else {
            stack.push('$');
        }
    }
    stack == vec!['#']
}
// stack
#[test]
fn test1_331() {
    assert_eq!(
        is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()),
        true
    );
    assert_eq!(is_valid_serialization("1,#".to_string()), false);
    assert_eq!(is_valid_serialization("9,#,#,1".to_string()), false);
}
