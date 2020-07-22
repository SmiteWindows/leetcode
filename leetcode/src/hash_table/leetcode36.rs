// https://leetcode.com/problems/valid-sudoku/
// Runtime: 4 ms
// Memory Usage: 2 MB
use std::collections::HashSet;
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];
    let mut boxes = vec![HashSet::new(); 9];
    for (i, row) in rows.iter_mut().enumerate().take(9) {
        for (j, col) in cols.iter_mut().enumerate().take(9) {
            let c = board[i][j];
            if c == '.' {
                continue;
            }
            if !row.insert(c) {
                return false;
            }
            if !col.insert(c) {
                return false;
            }
            let k = (i / 3) * 3 + (j / 3);
            if !boxes[k].insert(c) {
                return false;
            }
        }
    }
    true
}
// hash_table
#[test]
fn test1_36() {
    assert_eq!(
        is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]),
        true
    );
    assert_eq!(
        is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]),
        false
    );
}
