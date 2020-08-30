// https://leetcode-cn.com/problems/falling-squares/
pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// segment_tree ordered_map
#[test]
#[ignore]
fn test1_699() {
    assert_eq!(
        falling_squares(vec![vec![1, 2], vec![2, 3], vec![6, 1]]),
        vec![2, 5, 5]
    );
    assert_eq!(
        falling_squares(vec![vec![100, 100], vec![200, 100]]),
        vec![100, 100]
    );
}
