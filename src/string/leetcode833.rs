// https://leetcode.com/problems/find-and-replace-in-string/
pub fn find_replace_string(
    s: String,
    indexes: Vec<i32>,
    sources: Vec<String>,
    targets: Vec<String>,
) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_833() {
    assert_eq!(
        find_replace_string(
            String::from("abcd"),
            vec![0, 2],
            vec![String::from("a"), String::from("cd")],
            vec![String::from("eee"), String::from("ffff")]
        ),
        String::from("eeebffff")
    );
    assert_eq!(
        find_replace_string(
            String::from("abcd"),
            vec![0, 2],
            vec![String::from("ab"), String::from("ec")],
            vec![String::from("eee"), String::from("ffff")]
        ),
        String::from("eeecd")
    );
}
