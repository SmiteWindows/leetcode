// https://leetcode.com/problems/largest-1-bordered-square/
pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1139() {
    assert_eq!(
        largest1_bordered_square(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        9
    );
    assert_eq!(largest1_bordered_square(vec![vec![1, 1, 0, 0]]), 1);
}
