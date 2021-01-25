// https://leetcode-cn.com/problems/perfect-rectangle/
// Runtime: 36 ms
// Memory Usage: 3.8 MB
use std::collections::HashMap;
pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
    let x1 = rectangles.iter().map(|v| v[0]).min().unwrap();
    let y1 = rectangles.iter().map(|v| v[1]).min().unwrap();
    let x2 = rectangles.iter().map(|v| v[2]).max().unwrap();
    let y2 = rectangles.iter().map(|v| v[3]).max().unwrap();
    let area = (x2 - x1) * (y2 - y1);
    let sum: i32 = rectangles
        .iter()
        .map(|v| (v[2] - v[0]) * (v[3] - v[1]))
        .sum();
    if sum != area {
        return false;
    }
    let mut hm: HashMap<(i32, i32), usize> = HashMap::new();
    for v in rectangles {
        *hm.entry((v[0], v[1])).or_default() += 1;
        *hm.entry((v[2], v[3])).or_default() += 1;
        *hm.entry((v[0], v[3])).or_default() += 1;
        *hm.entry((v[2], v[1])).or_default() += 1;
    }
    for (k, v) in hm {
        if k == (x1, y1) || k == (x1, y2) || k == (x2, y1) || k == (x2, y2) {
            if v != 1 {
                return false;
            }
        } else if v % 2 != 0 {
            return false;
        }
    }
    true
}
// line_sweep
#[test]
fn test1_391() {
    use leetcode_prelude::vec2;
    assert_eq!(
        is_rectangle_cover(vec2![
            [1, 1, 3, 3],
            [3, 1, 4, 2],
            [3, 2, 4, 4],
            [1, 3, 2, 4],
            [2, 3, 3, 4]
        ]),
        true
    );
    assert_eq!(
        is_rectangle_cover(vec2![
            [1, 1, 2, 3],
            [1, 3, 2, 4],
            [3, 1, 4, 2],
            [3, 2, 4, 4]
        ]),
        false
    );
    assert_eq!(
        is_rectangle_cover(vec2![
            [1, 1, 3, 3],
            [3, 1, 4, 2],
            [1, 3, 2, 4],
            [3, 2, 4, 4]
        ]),
        false
    );
    assert_eq!(
        is_rectangle_cover(vec2![
            [1, 1, 3, 3],
            [3, 1, 4, 2],
            [1, 3, 2, 4],
            [2, 2, 4, 4]
        ]),
        false
    );
}
