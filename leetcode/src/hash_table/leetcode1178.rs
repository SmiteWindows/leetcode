// https://leetcode-cn.com/problems/number-of-valid-words-for-each-puzzle/
pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    todo!()
}
// bit_manipulation hash_table
#[test]
#[ignore]
fn test2_1178() {
    assert_eq!(
        find_num_of_valid_words(
            vec![
                String::from("aaaa"),
                String::from("asas"),
                String::from("able"),
                String::from("ability"),
                String::from("actt"),
                String::from("actor"),
                String::from("access")
            ],
            vec![
                String::from("aboveyz"),
                String::from("abrodyz"),
                String::from("abslute"),
                String::from("absoryz"),
                String::from("actresz"),
                String::from("gaswxyz")
            ],
        ),
        vec![1, 1, 3, 2, 4, 0]
    );
}
