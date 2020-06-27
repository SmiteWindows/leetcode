// https://leetcode.com/problems/valid-tic-tac-toe-state/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
    let mut a = vec![vec![]; 3];
    let mut x = 0;
    let mut o = 0;
    for (i, row) in board.iter().enumerate() {
        for c in row.chars() {
            match c {
                'X' => {
                    x += 1;
                }
                'O' => {
                    o += 1;
                }
                _ => {}
            }
            a[i].push(c);
        }
    }
    let win_x = win(&a, 'X');
    let win_o = win(&a, 'O');
    x == o + 1 && win_x >= 0 && win_o == 0 || x == o && win_x == 0 && win_o <= 1
}

fn win(board: &[Vec<char>], c: char) -> i32 {
    let mut rows = vec![0; 3];
    let mut cols = vec![0; 3];
    let mut diagonals = vec![0; 2];
    for i in 0..3 {
        for (j, col) in cols.iter_mut().enumerate().take(3) {
            let v = if board[i][j] == c { 1 } else { 0 };
            rows[i] += v;
            *col += v;
            if i == j {
                diagonals[0] += v;
            }
            if i + j == 2 {
                diagonals[1] += v;
            }
        }
    }
    let mut sum = 0;
    if rows.iter().any(|&row| row == 3) {
        sum += 1;
    }
    if cols.iter().any(|&col| col == 3) {
        sum += 1;
    }
    if diagonals.iter().any(|&diagonal| diagonal == 3) {
        sum += 1;
    }
    sum
}
// math recursion
#[test]
fn test2_794() {
    assert_eq!(
        valid_tic_tac_toe(vec![
            String::from("O  "),
            String::from("   "),
            String::from("   ")
        ]),
        false
    );
    assert_eq!(
        valid_tic_tac_toe(vec![
            String::from("XOX"),
            String::from(" X "),
            String::from("   ")
        ]),
        false
    );
    assert_eq!(
        valid_tic_tac_toe(vec![
            String::from("XXX"),
            String::from("   "),
            String::from("OOO")
        ]),
        false
    );
    assert_eq!(
        valid_tic_tac_toe(vec![
            String::from("XOX"),
            String::from("O O"),
            String::from("XOX")
        ]),
        true
    );
}
