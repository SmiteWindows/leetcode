// https://leetcode.com/problems/transform-to-chessboard/
pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// math array
#[test]
#[ignore]
fn test1_782() {
    assert_eq!(
        moves_to_chessboard(vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1]
        ]),
        2
    );
    assert_eq!(moves_to_chessboard(vec![vec![0, 1], vec![1, 0]]), 0);
    assert_eq!(moves_to_chessboard(vec![vec![1, 0], vec![1, 0]]), -1);
}
