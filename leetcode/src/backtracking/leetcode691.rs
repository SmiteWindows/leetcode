// https://leetcode-cn.com/problems/stickers-to-spell-word/
pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
    todo!()
}
// backtracking dynamic_programming
#[test]
#[ignore]
fn test1_691() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        min_stickers(
            vec_string!["with", "example", "science"],
            "thehat".to_string()
        ),
        3
    );
    assert_eq!(
        min_stickers(vec_string!["notice", "possible"], "basicbasic".to_string()),
        -1
    );
}
