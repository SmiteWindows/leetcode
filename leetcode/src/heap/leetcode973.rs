// https://leetcode-cn.com/problems/k-closest-points-to-origin/
// Runtime: 28 ms
// Memory Usage: 2.7 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let n = points.len();
    quick_select(&mut points, 0, n - 1, k as usize);
    points.resize(k as usize, vec![]);
    points
}

fn distance(v: &[i32]) -> i32 {
    v[0] * v[0] + v[1] * v[1]
}

fn quick_select(a: &mut Vec<Vec<i32>>, l: usize, r: usize, k: usize) {
    if l == r {
        return;
    }
    let index = partition(a, l, r);
    let rank = index - l + 1;
    match rank.cmp(&k) {
        Greater => quick_select(a, l, index - 1, k),
        Less => quick_select(a, index + 1, r, k - rank),
        Equal => {}
    }
}

fn partition(a: &mut Vec<Vec<i32>>, l: usize, r: usize) -> usize {
    let x = distance(&a[r]);
    let mut i = l;
    for j in l..r {
        if distance(&a[j]) <= x {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, r);
    i
}
// divide_and_conquer heap sort
#[test]
fn test2_973() {
    use leetcode_prelude::vec2;
    assert_eq!(k_closest(vec2![[1, 3], [-2, 2]], 1), vec2![[-2, 2]]);
    assert_eq!(
        k_closest(vec2![[3, 3], [5, -1], [-2, 4]], 2),
        vec2![[3, 3], [-2, 4]]
    );
}
