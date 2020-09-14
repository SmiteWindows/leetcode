// https://leetcode-cn.com/problems/set-intersection-size-at-least-two/
pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_757() {
    use leetcode_prelude::vec2;
    assert_eq!(
        intersection_size_two(vec2![[1, 3], [1, 4], [2, 5], [3, 5]]),
        3
    );
    assert_eq!(
        intersection_size_two(vec2![[1, 2], [2, 3], [2, 4], [4, 5]]),
        5
    );
}
