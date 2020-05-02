// https://leetcode.com/problems/top-k-frequent-words/
pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    todo!()
}
// hash_table trie heap
#[test]
#[ignore]
fn test3_692() {
    assert_eq!(
        top_k_frequent(
            vec![
                String::from("i"),
                String::from("love"),
                String::from("leetcode"),
                String::from("i"),
                String::from("love"),
                String::from("coding"),
            ],
            2
        ),
        vec![String::from("i"), String::from("love")]
    );
    assert_eq!(
        top_k_frequent(
            vec![
                String::from("the"),
                String::from("day"),
                String::from("is"),
                String::from("sunny"),
                String::from("the"),
                String::from("the"),
                String::from("the"),
                String::from("sunny"),
                String::from("is"),
                String::from("is"),
            ],
            4
        ),
        vec![
            String::from("the"),
            String::from("is"),
            String::from("sunny"),
            String::from("day")
        ]
    );
}
