// https://leetcode-cn.com/problems/maximal-rectangle/
pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    todo!()
}
// stack array hash_table dynamic_programming
#[test]
#[ignore]
fn test1_85() {
    use leetcode_prelude::vec2_char;
    assert_eq!(
        maximal_rectangle(vec2_char![
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '0', '1', '0']
        ]),
        6
    );
}
