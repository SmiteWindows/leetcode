// https://leetcode-cn.com/problems/number-of-islands/
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    todo!()
}
// depth_first_search breadth_first_search union_find
#[test]
#[ignore]
fn test3_200() {
    use leetcode_prelude::vec2_char;
    assert_eq!(
        num_islands(vec2_char![
            ['1', '1', '1', '1', '0'],
            ['1', '1', '0', '1', '0'],
            ['1', '1', '0', '0', '0'],
            ['0', '0', '0', '0', '0']
        ]),
        1
    );
    assert_eq!(
        num_islands(vec2_char![
            ['1', '1', '0', '0', '0'],
            ['1', '1', '0', '0', '0'],
            ['0', '0', '1', '0', '0'],
            ['0', '0', '0', '1', '1']
        ]),
        3
    );
}
