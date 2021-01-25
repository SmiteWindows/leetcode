// https://leetcode-cn.com/problems/minimum-number-of-arrows-to-burst-balloons/
// Runtime: 36 ms
// Memory Usage: 2.8 MB
pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    if points.is_empty() {
        return 0;
    }
    points.sort_by_key(|p| p[1]);
    let mut end = points[0][1];
    let mut res = 1;
    for point in points.iter().skip(1) {
        if point[0] <= end {
            continue;
        }
        end = point[1];
        res += 1;
    }
    res
}
// greedy
#[test]
fn test1_452() {
    use leetcode_prelude::vec2;
    assert_eq!(
        find_min_arrow_shots(vec2![[10, 16], [2, 8], [1, 6], [7, 12]]),
        2
    );
}
