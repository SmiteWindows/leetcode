// https://leetcode-cn.com/problems/number-of-ways-to-wear-different-hats-to-each-other/
pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// bit_manipulation dynamic_programming
#[test]
#[ignore]
fn test2_1434() {
    use leetcode_prelude::vec2;
    assert_eq!(number_ways(vec2![[3, 4], [4, 5], [5]]), 1);
    assert_eq!(number_ways(vec2![[3, 5, 1], [3, 5]]), 4);
    assert_eq!(
        number_ways(vec2![
            [1, 2, 3, 4],
            [1, 2, 3, 4],
            [1, 2, 3, 4],
            [1, 2, 3, 4]
        ]),
        24
    );
    assert_eq!(
        number_ways(vec2![
            [1, 2, 3],
            [2, 3, 5, 6],
            [1, 3, 7, 9],
            [1, 8, 9],
            [2, 5, 7]
        ]),
        111
    );
}
