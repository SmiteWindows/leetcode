// https://leetcode.com/problems/occurrences-after-bigram/
pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_1078() {
    assert_eq!(
        find_ocurrences(
            String::from("alice is a good girl she is a good student"),
            String::from("a"),
            String::from("good")
        ),
        vec![String::from("girl"), String::from("student")]
    );
    assert_eq!(
        find_ocurrences(
            String::from("we will we will rock you"),
            String::from("we"),
            String::from("will")
        ),
        vec![String::from("we"), String::from("rock")]
    );
}
