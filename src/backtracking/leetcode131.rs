// https://leetcode.com/problems/palindrome-partitioning/
pub fn partition(s: String) -> Vec<Vec<String>> {
    todo!()
}
// backtracking
#[test]
#[ignore]
fn test1_131() {
    assert_eq!(
        partition(String::from("aab")),
        vec![
            vec![String::from("aa"), String::from("b")],
            vec![String::from("a"), String::from("a"), String::from("b")]
        ]
    );
}
