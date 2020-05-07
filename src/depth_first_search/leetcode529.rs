// https://leetcode.com/problems/minesweeper/
pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
    todo!()
}
// depth_first_search breadth_first_search
#[test]
#[ignore]
fn test1_529() {
    assert_eq!(
        update_board(
            vec![
                vec!['E', 'E', 'E', 'E', 'E'],
                vec!['E', 'E', 'M', 'E', 'E'],
                vec!['E', 'E', 'E', 'E', 'E'],
                vec!['E', 'E', 'E', 'E', 'E']
            ],
            vec![3, 0]
        ),
        vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'M', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B']
        ]
    );
    assert_eq!(
        update_board(
            vec![
                vec!['B', '1', 'E', '1', 'B'],
                vec!['B', '1', 'M', '1', 'B'],
                vec!['B', '1', '1', '1', 'B'],
                vec!['B', 'B', 'B', 'B', 'B']
            ],
            vec![1, 2]
        ),
        vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'X', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B']
        ]
    );
}
