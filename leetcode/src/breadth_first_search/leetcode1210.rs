// https://leetcode-cn.com/problems/minimum-moves-to-reach-target-with-rotations/
pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1210() {
    assert_eq!(
        minimum_moves(vec![
            vec![0, 0, 0, 0, 0, 1],
            vec![1, 1, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 1, 0, 1, 0],
            vec![0, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0]
        ]),
        11
    );
    assert_eq!(
        minimum_moves(vec![
            vec![0, 0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 1, 1],
            vec![1, 1, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 1],
            vec![1, 1, 1, 0, 0, 0]
        ]),
        9
    );
}
