// https://leetcode-cn.com/problems/minimum-moves-to-move-a-box-to-their-target-location/
pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1263() {
    assert_eq!(
        min_push_box(vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '#', '#', '#', '#'],
            vec!['#', '.', '.', 'B', '.', '#'],
            vec!['#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#']
        ]),
        3
    );
    assert_eq!(
        min_push_box(vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '#', '#', '#', '#'],
            vec!['#', '.', '.', 'B', '.', '#'],
            vec!['#', '#', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#']
        ]),
        -1
    );
    assert_eq!(
        min_push_box(vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '.', '.', '#', '#'],
            vec!['#', '.', '#', 'B', '.', '#'],
            vec!['#', '.', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#']
        ]),
        5
    );
    assert_eq!(
        min_push_box(vec![
            vec!['#', '#', '#', '#', '#', '#', '#'],
            vec!['#', 'S', '#', '.', 'B', 'T', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#']
        ]),
        -1
    );
}
