// https://leetcode-cn.com/problems/k-closest-points-to-origin/
pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    todo!()
}
// divide_and_conquer heap sort
#[test]
#[ignore]
fn test1_973() {
    use leetcode_prelude::vec2;
    assert_eq!(k_closest(vec2![[1, 3], [-2, 2]], 1), vec2![[-2, 2]]);
    assert_eq!(
        k_closest(vec2![[3, 3], [5, -1], [-2, 4]], 2),
        vec2![[3, 3], [-2, 4]]
    );
}
