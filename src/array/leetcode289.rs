// https://leetcode.com/problems/game-of-life/
pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_289() {
    let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    game_of_life(&mut board);
    assert_eq!(
        board,
        vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]
    );
}
