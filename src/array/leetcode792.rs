// https://leetcode.com/problems/number-of-matching-subsequences/
pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_792() {
    assert_eq!(
        num_matching_subseq(
            String::from("abcde"),
            vec![
                String::from("a"),
                String::from("bb"),
                String::from("acd"),
                String::from("ace")
            ]
        ),
        3
    );
}
