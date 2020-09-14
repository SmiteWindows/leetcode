// https://leetcode-cn.com/problems/special-positions-in-a-binary-matrix/
pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1582() {
    use leetcode_prelude::vec2;
    assert_eq!(num_special(vec2![[1, 0, 0], [0, 0, 1], [1, 0, 0]]), 1);
    assert_eq!(num_special(vec2![[1, 0, 0], [0, 1, 0], [0, 0, 1]]), 3);
    assert_eq!(
        num_special(vec2![
            [0, 0, 0, 1],
            [1, 0, 0, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0]
        ]),
        2
    );
    assert_eq!(
        num_special(vec2![
            [0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 1, 1]
        ]),
        3
    );
}
