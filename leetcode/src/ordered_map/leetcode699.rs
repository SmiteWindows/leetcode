// https://leetcode-cn.com/problems/falling-squares/
pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// segment_tree ordered_map
#[test]
#[ignore]
fn test2_699() {
    use leetcode_prelude::vec2;
    assert_eq!(
        falling_squares(vec2![[1, 2], [2, 3], [6, 1]]),
        vec![2, 5, 5]
    );
    assert_eq!(
        falling_squares(vec2![[100, 100], [200, 100]]),
        vec![100, 100]
    );
}
