// https://leetcode-cn.com/problems/frog-position-after-t-seconds/
pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_1377() {
    use leetcode_prelude::{assert_approx_eq, vec2};
    assert_approx_eq!(
        frog_position(
            7,
            vec2![[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]],
            2,
            4
        ),
        0.16666666666666666
    );
    assert_approx_eq!(
        frog_position(
            7,
            vec2![[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]],
            1,
            7
        ),
        0.3333333333333333
    );
    assert_approx_eq!(
        frog_position(
            7,
            vec2![[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]],
            20,
            6
        ),
        0.16666666666666666
    );
}
