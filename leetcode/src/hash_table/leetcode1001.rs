// https://leetcode-cn.com/problems/grid-illumination/
pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_1001() {
    assert_eq!(
        grid_illumination(
            5,
            vec![vec![0, 0], vec![4, 4]],
            vec![vec![1, 1], vec![1, 0]]
        ),
        vec![1, 0]
    );
}
