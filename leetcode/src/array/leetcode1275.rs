// https://leetcode-cn.com/problems/find-winner-on-a-tic-tac-toe-game/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
    let mut board = [[' '; 3]; 3];
    let mut player = 'X';
    let n = moves.len();
    for m in moves {
        let r = m[0] as usize;
        let c = m[1] as usize;
        board[r][c] = player;
        player = if player == 'X' { 'O' } else { 'X' };
    }
    let ss = [
        [(0, 0), (0, 1), (0, 2)],
        [(1, 0), (1, 1), (1, 2)],
        [(2, 0), (2, 1), (2, 2)],
        [(0, 0), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1)],
        [(0, 2), (1, 2), (2, 2)],
        [(0, 0), (1, 1), (2, 2)],
        [(0, 2), (1, 1), (2, 0)],
    ];
    for s in &ss {
        let mut a = 0;
        let mut b = 0;
        for p in s {
            match board[p.0][p.1] {
                'X' => a += 1,
                'O' => b += 1,
                _ => {}
            }
        }
        if a == 3 {
            return "A".to_string();
        }
        if b == 3 {
            return "B".to_string();
        }
    }
    if n == 9 {
        return "Draw".to_string();
    }
    "Pending".to_string()
}
// array
#[test]
fn test1_1275() {
    use leetcode_prelude::vec2;
    assert_eq!(
        tictactoe(vec2![[0, 0], [2, 0], [1, 1], [2, 1], [2, 2]]),
        "A".to_string()
    );
    assert_eq!(
        tictactoe(vec2![[0, 0], [1, 1], [0, 1], [0, 2], [1, 0], [2, 0]]),
        "B".to_string()
    );
    assert_eq!(
        tictactoe(vec2![
            [0, 0],
            [1, 1],
            [2, 0],
            [1, 0],
            [1, 2],
            [2, 1],
            [0, 1],
            [0, 2],
            [2, 2]
        ]),
        "Draw".to_string()
    );
    assert_eq!(tictactoe(vec2![[0, 0], [1, 1]]), "Pending".to_string());
}
