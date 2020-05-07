// https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/
pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1275() {
    assert_eq!(
        tictactoe(vec![
            vec![0, 0],
            vec![2, 0],
            vec![1, 1],
            vec![2, 1],
            vec![2, 2]
        ]),
        String::from("A")
    );
    assert_eq!(
        tictactoe(vec![
            vec![0, 0],
            vec![1, 1],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![2, 0]
        ]),
        String::from("B")
    );
    assert_eq!(
        tictactoe(vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 0],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![0, 1],
            vec![0, 2],
            vec![2, 2]
        ]),
        String::from("Draw")
    );
    assert_eq!(
        tictactoe(vec![vec![0, 0], vec![1, 1]]),
        String::from("Pending")
    );
}
