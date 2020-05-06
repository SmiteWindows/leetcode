// https://leetcode.com/problems/verifying-an-alien-dictionary/
pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_953() {
    assert_eq!(
        is_alien_sorted(
            vec![String::from("hello"), String::from("leetcode")],
            String::from("hlabcdefgijkmnopqrstuvwxyz")
        ),
        true
    );
    assert_eq!(
        is_alien_sorted(
            vec![
                String::from("word"),
                String::from("world"),
                String::from("row")
            ],
            String::from("worldabcefghijkmnpqstuvxyz")
        ),
        false
    );
    assert_eq!(
        is_alien_sorted(
            vec![String::from("apple"), String::from("app")],
            String::from("abcdefghijklmnopqrstuvwxyz")
        ),
        false
    );
}
