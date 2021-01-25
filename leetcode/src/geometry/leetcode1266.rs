// https://leetcode-cn.com/problems/minimum-time-visiting-all-points/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let n = points.len();
    for i in 1..n {
        let x1 = points[i - 1][0];
        let y1 = points[i - 1][1];
        let x2 = points[i][0];
        let y2 = points[i][1];
        res += (x2 - x1).abs().max((y2 - y1).abs());
    }
    res
}
// geometry array
#[test]
fn test1_1266() {
    use leetcode_prelude::vec2;
    assert_eq!(
        min_time_to_visit_all_points(vec2![[1, 1], [3, 4], [-1, 0]]),
        7
    );
    assert_eq!(min_time_to_visit_all_points(vec2![[3, 2], [-2, 2]]), 5);
}
