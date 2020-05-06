// https://leetcode.com/problems/longest-string-chain/
pub fn longest_str_chain(words: Vec<String>) -> i32 {
    todo!()
}
// hash_table dynamic_programming
#[test]
#[ignore]
fn test1_1048() {
    assert_eq!(
        longest_str_chain(vec![
            String::from("a"),
            String::from("b"),
            String::from("ba"),
            String::from("bca"),
            String::from("bda"),
            String::from("bdca")
        ]),
        4
    );
}
