// https://leetcode.com/problems/island-perimeter/
pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_463() {
    assert_eq!(
        island_perimeter(vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0]
        ]),
        16
    );
}
