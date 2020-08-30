// https://leetcode-cn.com/problems/bricks-falling-when-hit/
pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// union_find
#[test]
#[ignore]
fn test1_803() {
    assert_eq!(
        hit_bricks(vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0]], vec![vec![1, 0]]),
        vec![2]
    );
    assert_eq!(
        hit_bricks(
            vec![vec![1, 0, 0, 0], vec![1, 1, 0, 0]],
            vec![vec![1, 1], vec![1, 0]]
        ),
        vec![0, 0]
    );
}
