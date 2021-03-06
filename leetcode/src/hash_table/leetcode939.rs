// https://leetcode-cn.com/problems/minimum-area-rectangle/
// Runtime: 228 ms
// Memory Usage: 2.2 MB
use std::collections::HashSet;
pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut hs = HashSet::new();
    for point in points.iter().take(n) {
        hs.insert((point[0], point[1]));
    }
    let mut min = i32::MAX;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let x1 = points[i][0];
            let y1 = points[i][1];
            let x2 = points[j][0];
            let y2 = points[j][1];
            if x2 != x1 && y2 != y1 && hs.contains(&(x1, y2)) && hs.contains(&(x2, y1)) {
                min = min.min((x2 - x1).abs() * (y2 - y1).abs())
            }
        }
    }
    if min == i32::MAX {
        0
    } else {
        min
    }
}
// hash_table
#[test]
fn test1_939() {
    use leetcode_prelude::vec2;
    assert_eq!(
        min_area_rect(vec2![[1, 1], [1, 3], [3, 1], [3, 3], [2, 2]]),
        4
    );
    assert_eq!(
        min_area_rect(vec2![[1, 1], [1, 3], [3, 1], [3, 3], [4, 1], [4, 3]]),
        2
    );
}
