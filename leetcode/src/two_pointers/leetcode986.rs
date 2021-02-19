// https://leetcode-cn.com/problems/interval-list-intersections/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
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
    use leetcode_prelude::vec2;
    assert_eq!(
        interval_intersection(
            vec2![[0, 2], [5, 10], [13, 23], [24, 25]],
            vec2![[1, 5], [8, 12], [15, 24], [25, 26]]
        ),
        vec2![[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
    );
}
