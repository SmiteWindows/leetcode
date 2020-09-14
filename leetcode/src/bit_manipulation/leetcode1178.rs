// https://leetcode-cn.com/problems/number-of-valid-words-for-each-puzzle/
pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    todo!()
}
// bit_manipulation hash_table
#[test]
#[ignore]
fn test1_1178() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        find_num_of_valid_words(
            vec_string!["aaaa", "asas", "able", "ability", "actt", "actor", "access"],
            vec_string!["aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz"],
        ),
        vec![1, 1, 3, 2, 4, 0]
    );
}
