// https://leetcode.com/problems/verifying-an-alien-dictionary/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let mut mapping = vec![0 as char; 256];
    for (i, c) in order.chars().enumerate() {
        mapping[c as usize] = (i as u8 + b'a') as char;
    }
    let words = words
        .into_iter()
        .map(|s| translate(s, &mapping))
        .collect::<Vec<_>>();
    let mut sorted = words.to_vec();
    sorted.sort();
    words == sorted
}

fn translate(s: String, mapping: &[char]) -> String {
    s.chars().map(|c| mapping[c as usize]).collect()
}
// hash_table
#[test]
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
