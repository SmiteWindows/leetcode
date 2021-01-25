// https://leetcode-cn.com/problems/available-captures-for-rook/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let rook = search_rook(&board);
    for direction in vec![
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ] {
        if search_pawn(&board, &rook, direction) {
            sum += 1;
        }
    }
    sum
}

struct Chess {
    r: usize,
    c: usize,
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn search_rook(board: &[Vec<char>]) -> Chess {
    for (r, row) in board.iter().enumerate().take(8) {
        for (c, &col) in row.iter().enumerate().take(8) {
            if col == 'R' {
                return Chess { r, c };
            }
        }
    }
    unreachable!()
}

fn search_pawn(board: &[Vec<char>], rook: &Chess, direction: Direction) -> bool {
    let mut r = rook.r;
    let mut c = rook.c;
    match direction {
        Direction::Right => {
            while c + 1 < 8 && board[r][c + 1] != 'B' {
                c += 1;
                if board[r][c] == 'p' {
                    return true;
                }
            }
        }
        Direction::Down => {
            while r + 1 < 8 && board[r + 1][c] != 'B' {
                r += 1;
                if board[r][c] == 'p' {
                    return true;
                }
            }
        }
        Direction::Left => {
            while c > 0 && board[r][c - 1] != 'B' {
                c -= 1;
                if board[r][c] == 'p' {
                    return true;
                }
            }
        }
        Direction::Up => {
            while r > 0 && board[r - 1][c] != 'B' {
                r -= 1;
                if board[r][c] == 'p' {
                    return true;
                }
            }
        }
    }
    false
}
// array
#[test]
fn test1_999() {
    use leetcode_prelude::vec2_char;
    assert_eq!(
        num_rook_captures(vec2_char![
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', 'p', '.', '.', '.', '.'],
            ['.', '.', '.', 'R', '.', '.', '.', 'p'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', 'p', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        3
    );
    assert_eq!(
        num_rook_captures(vec2_char![
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
            ['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
            ['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
            ['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
            ['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        0
    );
    assert_eq!(
        num_rook_captures(vec2_char![
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', 'p', '.', '.', '.', '.'],
            ['.', '.', '.', 'p', '.', '.', '.', '.'],
            ['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', 'B', '.', '.', '.', '.'],
            ['.', '.', '.', 'p', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        3
    );
}
