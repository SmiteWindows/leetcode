// https://leetcode-cn.com/problems/battleships-in-a-board/
// Runtime: 0 ms
// Memory Usage: 3.9 MB
pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let n = board.len();
    let m = board[0].len();
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if is_head(i, j, &board) {
                res += 1;
            }
        }
    }
    res
}

fn is_head(i: usize, j: usize, board: &[Vec<char>]) -> bool {
    if board[i][j] == '.' {
        return false;
    }
    if i > 0 && board[i - 1][j] == 'X' {
        return false;
    }
    if j > 0 && board[i][j - 1] == 'X' {
        return false;
    }
    true
}
#[test]
fn test419() {
    use leetcode_prelude::vec2_char;
    assert_eq!(
        count_battleships(vec2_char![
            ['X', '.', '.', 'X'],
            ['.', '.', '.', 'X'],
            ['.', '.', '.', 'X']
        ]),
        2
    );
}
