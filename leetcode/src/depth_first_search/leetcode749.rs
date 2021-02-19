// https://leetcode-cn.com/problems/contain-virus/
pub fn contain_virus(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_749() {
    use leetcode_prelude::vec2;
    assert_eq!(
        contain_virus(vec2![
            [0, 1, 0, 0, 0, 0, 0, 1],
            [0, 1, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 0]
        ]),
        10
    );
    assert_eq!(contain_virus(vec2![[1, 1, 1], [1, 0, 1], [1, 1, 1]]), 4);
    assert_eq!(
        contain_virus(vec2![
            [1, 1, 1, 0, 0, 0, 0, 0, 0],
            [1, 0, 1, 0, 1, 1, 1, 1, 1],
            [1, 1, 1, 0, 0, 0, 0, 0, 0]
        ]),
        13
    );
}
