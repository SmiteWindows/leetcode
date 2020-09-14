// https://leetcode-cn.com/problems/valid-sudoku/
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
    use leetcode_prelude::vec2_char;
    assert_eq!(
        is_valid_sudoku(vec2_char![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]),
        true
    );
    assert_eq!(
        is_valid_sudoku(vec2_char![
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]),
        false
    );
}
