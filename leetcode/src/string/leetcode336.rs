// https://leetcode.com/problems/palindrome-pairs/
pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
    todo!()
}
// hash_table string trie
#[test]
#[ignore]
fn test3_336() {
    assert_eq!(
        palindrome_pairs(vec![
            String::from("abcd"),
            String::from("dcba"),
            String::from("lls"),
            String::from("s"),
            String::from("sssll"),
        ]),
        vec![vec![0, 1], vec![1, 0], vec![3, 2], vec![2, 4]]
    );
    assert_eq!(
        palindrome_pairs(vec![
            String::from("bat"),
            String::from("tab"),
            String::from("cat"),
        ]),
        vec![vec![0, 1], vec![1, 0]]
    );
}
