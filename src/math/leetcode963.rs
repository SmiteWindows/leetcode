// https://leetcode.com/problems/minimum-area-rectangle-ii/
// Runtime: 4 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
    let mut hm: HashMap<(i32, i32, i32), Vec<(i32, i32)>> = HashMap::new();
    let n = points.len();
    let mut res = f64::MAX;
    for i in 0..n {
        for j in i + 1..n {
            let x1 = points[i][0];
            let x2 = points[j][0];
            let y1 = points[i][1];
            let y2 = points[j][1];
            let c = (
                x1 + x2,
                y1 + y2,
                (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1),
            );
            let points = hm.entry(c).or_default();
            for point in points.iter() {
                let e1 = edge(x1, y1, point.0, point.1);
                let e2 = edge(x2, y2, point.0, point.1);
                res = res.min(e1 * e2);
            }
            points.push((x1, y1));
        }
    }
    if res == f64::MAX {
        0.0
    } else {
        res
    }
}

fn edge(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    (((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)) as f64).sqrt()
}
// math geometry
#[test]
fn test1_963() {
    assert_eq!(
        min_area_free_rect(vec![vec![1, 2], vec![2, 1], vec![1, 0], vec![0, 1]]),
        2.0000000000000004
    );
    assert_eq!(
        min_area_free_rect(vec![
            vec![0, 1],
            vec![2, 1],
            vec![1, 1],
            vec![1, 0],
            vec![2, 0]
        ]),
        1.00000
    );
    assert_eq!(
        min_area_free_rect(vec![
            vec![0, 3],
            vec![1, 2],
            vec![3, 1],
            vec![1, 3],
            vec![2, 1]
        ]),
        0.00000
    );
    assert_eq!(
        min_area_free_rect(vec![
            vec![3, 1],
            vec![1, 1],
            vec![0, 1],
            vec![2, 1],
            vec![3, 3],
            vec![3, 2],
            vec![0, 2],
            vec![2, 3]
        ]),
        2.00000
    );
}
