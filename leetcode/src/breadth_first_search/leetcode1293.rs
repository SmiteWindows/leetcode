// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/
pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1293() {
    assert_eq!(
        shortest_path(
            vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![0, 0, 0],
                vec![0, 1, 1],
                vec![0, 0, 0]
            ],
            1
        ),
        6
    );
    assert_eq!(
        shortest_path(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 0, 0]], 1),
        -1
    );
}
