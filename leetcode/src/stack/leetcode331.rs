// https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn is_valid_serialization(preorder: String) -> bool {
    let mut stack = vec![];
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
        is_valid_serialization(String::from("9,3,4,#,#,1,#,#,2,#,6,#,#")),
        true
    );
    assert_eq!(is_valid_serialization(String::from("1,#")), false);
    assert_eq!(is_valid_serialization(String::from("9,#,#,1")), false);
}
