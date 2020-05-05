// https://leetcode.com/problems/stickers-to-spell-word/
pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
    todo!()
}
// backtracking dynamic_programming
#[test]
#[ignore]
fn test1_691() {
    assert_eq!(
        min_stickers(
            vec![
                String::from("with"),
                String::from("example"),
                String::from("science")
            ],
            String::from("thehat")
        ),
        3
    );
    assert_eq!(
        min_stickers(
            vec![String::from("notice"), String::from("possible")],
            String::from("basicbasic")
        ),
        -1
    );
}
