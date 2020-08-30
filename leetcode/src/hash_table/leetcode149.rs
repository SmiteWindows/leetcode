// https://leetcode-cn.com/problems/max-points-on-a-line/
// Runtime: 8 ms
// Memory Usage: 2 MB
use std::collections::HashMap;
pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    if n == 0 {
        return 0;
    }
    let mut res = 1;
    for i in 0..n {
        res = res.max(from_point(i, &points));
    }
    res as i32
}

fn from_point(i: usize, points: &[Vec<i32>]) -> usize {
    let n = points.len();
    let x1 = points[i][0];
    let y1 = points[i][1];
    let mut hm: HashMap<(i32, i32), usize> = HashMap::new();
    let mut origin = 0;
    for point in points.iter().take(n) {
        let x2 = point[0];
        let y2 = point[1];
        let mut dx = x2 - x1;
        let mut dy = y2 - y1;
        if dx == 0 && dy == 0 {
            origin += 1;
        } else {
            if dx == 0 {
                *hm.entry((0, 1)).or_default() += 1;
                continue;
            }
            if dy == 0 {
                *hm.entry((1, 0)).or_default() += 1;
                continue;
            }
            if dy < 0 {
                dy *= -1;
                dx *= -1;
            }
            let z = gcd(dx, dy);
            dy /= z;
            dx /= z;
            *hm.entry((dx, dy)).or_default() += 1;
        }
    }
    hm.values().max().unwrap_or(&0) + origin
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
    while m != 0 {
        let temp = m;
        m = n % temp;
        n = temp;
    }
    n.abs()
}
// math hash_table
#[test]
fn test2_149() {
    assert_eq!(max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 3);
    assert_eq!(
        max_points(vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4]
        ]),
        4
    );
}
