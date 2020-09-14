// https://leetcode-cn.com/problems/erect-the-fence/
pub fn outer_trees(points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    todo!()
}
// geometry
#[test]
#[ignore]
fn test1_587() {
    use leetcode_prelude::vec2;
    assert_eq!(
        outer_trees(vec2![[1, 1], [2, 2], [2, 0], [2, 4], [3, 3], [4, 2]]),
        vec2![[1, 1], [2, 0], [4, 2], [3, 3], [2, 4]]
    );
    assert_eq!(
        outer_trees(vec2![[1, 2], [2, 2], [4, 2]]),
        vec2![[1, 2], [2, 2], [4, 2]]
    );
}
