// https://leetcode-cn.com/problems/interval-list-intersections/
#![allow(clippy::many_single_char_names)]
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut i = 0;
    let mut j = 0;
    let n = a.len();
    let m = b.len();
    while i < n && j < m {
        let al = a[i][0];
        let ar = a[i][1];
        let bl = b[j][0];
        let br = b[j][1];
        if ar < bl {
            i += 1;
            continue;
        }
        if br < al {
            j += 1;
            continue;
        }
        let l = al.max(bl);
        let r = ar.min(br);
        res.push(vec![l, r]);
        if ar < br {
            i += 1;
        } else {
            j += 1;
        }
    }
    res
}
// two_pointers
#[test]
fn test1_986() {
    assert_eq!(
        interval_intersection(
            vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
            vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
        ),
        vec![
            vec![1, 2],
            vec![5, 5],
            vec![8, 10],
            vec![15, 23],
            vec![24, 24],
            vec![25, 25]
        ]
    );
}
