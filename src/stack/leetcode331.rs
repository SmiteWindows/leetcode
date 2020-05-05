// https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/
pub fn is_valid_serialization(preorder: String) -> bool {
    todo!()
}
// stack
#[test]
#[ignore]
fn test1_331() {
    assert_eq!(
        is_valid_serialization(String::from("9,3,4,#,#,1,#,#,2,#,6,#,#")),
        true
    );
    assert_eq!(is_valid_serialization(String::from("1,#")), false);
    assert_eq!(is_valid_serialization(String::from("9,#,#,1")), false);
}
