// https://leetcode.com/problems/minimum-time-visiting-all-points/
pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let n = points.len();
    for i in 1..n {
        let x1 = points[i - 1][0];
        let y1 = points[i - 1][1];
        let x2 = points[i][0];
        let y2 = points[i][1];
        res += i32::max((x2 - x1).abs(), (y2 - y1).abs());
    }
    res
}
// geometry array
#[test]
fn test2_1266() {
    assert_eq!(
        min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
        7
    );
    assert_eq!(
        min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
        5
    );
}
