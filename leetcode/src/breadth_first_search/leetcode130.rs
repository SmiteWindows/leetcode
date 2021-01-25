// https://leetcode-cn.com/problems/surrounded-regions/
pub fn solve(board: &mut Vec<Vec<char>>) {
    todo!()
}
// depth_first_search breadth_first_search union_find
#[test]
#[ignore]
fn test2_130() {
    let mut board = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    let res = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    solve(&mut board);
    assert_eq!(board, res);
}
