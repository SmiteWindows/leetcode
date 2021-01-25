// https://leetcode-cn.com/problems/minimum-absolute-difference/
// Runtime: 20 ms
// Memory Usage: 3.2 MB
pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    arr.sort_unstable();
    let min = arr.windows(2).fold(i32::MAX, |x, v| x.min(v[1] - v[0]));
    let mut res: Vec<Vec<i32>> = vec![];
    for v in arr.windows(2) {
        if v[1] - v[0] == min {
            res.push(v.to_vec())
        }
    }
    res
}
// array
#[test]
fn test1_1200() {
    use leetcode_prelude::vec2;
    assert_eq!(
        minimum_abs_difference(vec![4, 2, 1, 3]),
        vec2![[1, 2], [2, 3], [3, 4]]
    );
    assert_eq!(minimum_abs_difference(vec![1, 3, 6, 10, 15]), vec2![[1, 3]]);
    assert_eq!(
        minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
        vec2![[-14, -10], [19, 23], [23, 27]]
    );
}
