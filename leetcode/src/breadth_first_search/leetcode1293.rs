// https://leetcode-cn.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/
pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1293() {
    use leetcode_prelude::vec2;
    assert_eq!(
        shortest_path(
            vec2![[0, 0, 0], [1, 1, 0], [0, 0, 0], [0, 1, 1], [0, 0, 0]],
            1
        ),
        6
    );
    assert_eq!(shortest_path(vec2![[0, 1, 1], [1, 1, 1], [1, 0, 0]], 1), -1);
}
