// https://leetcode.com/problems/detect-cycles-in-2d-grid/
pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_1559() {
    use leetcode_prelude::vec2_char;
    assert_eq!(
        contains_cycle(vec2_char![
            ['a', 'a', 'a', 'a'],
            ['a', 'b', 'b', 'a'],
            ['a', 'b', 'b', 'a'],
            ['a', 'a', 'a', 'a']
        ]),
        true
    );
    assert_eq!(
        contains_cycle(vec2_char![
            ['c', 'c', 'c', 'a'],
            ['c', 'd', 'c', 'c'],
            ['c', 'c', 'e', 'c'],
            ['f', 'c', 'c', 'c']
        ]),
        true
    );
    assert_eq!(
        contains_cycle(vec2_char![
            ['a', 'b', 'b'],
            ['b', 'z', 'b'],
            ['b', 'b', 'a']
        ]),
        false
    );
}
