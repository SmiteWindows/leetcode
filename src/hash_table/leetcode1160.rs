// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/
pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    todo!()
}
// hash_table array
#[test]
#[ignore]
fn test1_1160() {
    assert_eq!(
        count_characters(
            vec![
                String::from("cat"),
                String::from("bt"),
                String::from("hat"),
                String::from("tree")
            ],
            String::from("atach")
        ),
        6
    );
    assert_eq!(
        count_characters(
            vec![
                String::from("hello"),
                String::from("world"),
                String::from("leetcode")
            ],
            String::from("welldonehoneyr")
        ),
        10
    );
}
