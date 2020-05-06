// https://leetcode.com/problems/alphabet-board-path/
pub fn alphabet_board_path(target: String) -> String {
    todo!()
}
// hash_table string
#[test]
#[ignore]
fn test2_1138() {
    assert_eq!(
        alphabet_board_path(String::from("leet")),
        String::from("DDR!UURRR!!DDD!")
    );
    assert_eq!(
        alphabet_board_path(String::from("code")),
        String::from("RR!DDRR!UUL!R!")
    );
}
