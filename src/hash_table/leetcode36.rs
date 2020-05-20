// https://leetcode.com/problems/valid-sudoku/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::collections::HashSet;
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    for i in 0..9 {
        for j in 0..9 {
            let c: char = board[i][j];
            if c == '.' {
                continue;
            }
            if !rows[i].insert(c) {
                return false;
            }
            if !cols[j].insert(c) {
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
