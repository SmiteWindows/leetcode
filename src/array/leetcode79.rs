// https://leetcode.com/problems/word-search/
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    todo!()
}
// backtracking array
#[test]
#[ignore]
fn test2_79() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert_eq!(exist(board, String::from("ABCCED")), true);
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert_eq!(exist(board, String::from("SEE")), true);
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert_eq!(exist(board, String::from("ABCB")), false);
}
