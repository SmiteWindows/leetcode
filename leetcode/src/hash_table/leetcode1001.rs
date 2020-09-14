// https://leetcode-cn.com/problems/grid-illumination/
pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_1001() {
    use leetcode_prelude::vec2;
    assert_eq!(
        grid_illumination(5, vec2![[0, 0], [4, 4]], vec2![[1, 1], [1, 0]]),
        vec![1, 0]
    );
}
