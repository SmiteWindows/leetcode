// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
// Runtime: 36 ms
// Memory Usage: 2.8 MB
pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points;
    let n = points.len();
    if n == 0 {
        return 0;
    }
    points.sort_by_key(|p| p[1]);
    let mut end = points[0][1];
    let mut res = 1;
    for i in 1..n {
        if points[i][0] <= end {
            continue;
        }
        end = points[i][1];
        res += 1;
    }
    res
}
// greedy
#[test]
fn test1_452() {
    assert_eq!(
        find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
        2
    );
}
