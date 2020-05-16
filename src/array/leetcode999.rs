// https://leetcode.com/problems/available-captures-for-rook/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
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

pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    fn search_rook(board: &[Vec<char>]) -> Chess {
        for r in 0..8 {
            for c in 0..8 {
                if board[r][c] == 'R' {
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

    let mut sum = 0;
    let rook: Chess = search_rook(&board);
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
// array
#[test]
fn test1_999() {
    assert_eq!(
        num_rook_captures(vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        3
    );
    assert_eq!(
        num_rook_captures(vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        0
    );
    assert_eq!(
        num_rook_captures(vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        3
    );
}
