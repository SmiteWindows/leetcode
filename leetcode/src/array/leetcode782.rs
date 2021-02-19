// https://leetcode-cn.com/problems/transform-to-chessboard/
pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// math array
#[test]
#[ignore]
fn test2_782() {
    use leetcode_prelude::vec2;
    assert_eq!(
        moves_to_chessboard(vec2![
            [0, 1, 1, 0],
            [0, 1, 1, 0],
            [1, 0, 0, 1],
            [1, 0, 0, 1]
        ]),
        2
    );
    assert_eq!(moves_to_chessboard(vec2![[0, 1], [1, 0]]), 0);
    assert_eq!(moves_to_chessboard(vec2![[1, 0], [1, 0]]), -1);
}
