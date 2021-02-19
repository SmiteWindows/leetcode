// https://leetcode.com/problems/strange-printer-ii/

pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_1591() {
    use leetcode_prelude::vec2;
    assert_eq!(
        is_printable(vec2![
            [1, 1, 1, 1],
            [1, 2, 2, 1],
            [1, 2, 2, 1],
            [1, 1, 1, 1]
        ]),
        true
    );
    assert_eq!(
        is_printable(vec2![
            [1, 1, 1, 1],
            [1, 1, 3, 3],
            [1, 1, 3, 4],
            [5, 5, 1, 4]
        ]),
        true
    );
    assert_eq!(is_printable(vec2![[1, 2, 1], [2, 1, 2], [1, 2, 1]]), false);
    assert_eq!(is_printable(vec2![[1, 1, 1], [3, 1, 3]]), false);
}
