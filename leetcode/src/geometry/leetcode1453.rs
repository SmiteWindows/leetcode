// https://leetcode-cn.com/problems/maximum-number-of-darts-inside-of-a-circular-dartboard/
pub fn num_points(points: Vec<Vec<i32>>, r: i32) -> i32 {
    todo!()
}
// geometry
#[test]
#[ignore]
fn test1_1453() {
    use leetcode_prelude::vec2;
    assert_eq!(num_points(vec2![[-2, 0], [2, 0], [0, 2], [0, -2]], 2), 4);
    assert_eq!(
        num_points(vec2![[-3, 0], [3, 0], [2, 6], [5, 4], [0, 9], [7, 8]], 5),
        5
    );
    assert_eq!(num_points(vec2![[-2, 0], [2, 0], [0, 2], [0, -2]], 1), 1);
    assert_eq!(
        num_points(vec2![[1, 2], [3, 5], [1, -1], [2, 3], [4, 1], [1, 3]], 2),
        4
    );
}
