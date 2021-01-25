// https://leetcode-cn.com/problems/minimum-moves-to-reach-target-with-rotations/
pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1210() {
    use leetcode_prelude::vec2;
    assert_eq!(
        minimum_moves(vec2![
            [0, 0, 0, 0, 0, 1],
            [1, 1, 0, 0, 1, 0],
            [0, 0, 0, 0, 1, 1],
            [0, 0, 1, 0, 1, 0],
            [0, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 0, 0]
        ]),
        11
    );
    assert_eq!(
        minimum_moves(vec2![
            [0, 0, 1, 1, 1, 1],
            [0, 0, 0, 0, 1, 1],
            [1, 1, 0, 0, 0, 1],
            [1, 1, 1, 0, 0, 1],
            [1, 1, 1, 0, 0, 1],
            [1, 1, 1, 0, 0, 0]
        ]),
        9
    );
}
