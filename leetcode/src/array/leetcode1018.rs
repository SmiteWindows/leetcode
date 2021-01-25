// https://leetcode-cn.com/problems/binary-prefix-divisible-by-5/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
    let mut x = 0;
    let n = a.len();
    let mut res = vec![false; n];
    for (i, val) in a.iter().enumerate() {
        x = (x * 2 + val) % 5;
        res[i] = x == 0;
    }
    res
}
// array
#[test]
fn test1_1018() {
    assert_eq!(prefixes_div_by5(vec![0, 1, 1]), vec![true, false, false]);
    assert_eq!(prefixes_div_by5(vec![1, 1, 1]), vec![false, false, false]);
    assert_eq!(
        prefixes_div_by5(vec![0, 1, 1, 1, 1, 1]),
        vec![true, false, false, false, true, false]
    );
    assert_eq!(
        prefixes_div_by5(vec![1, 1, 1, 0, 1]),
        vec![false, false, false, false, false]
    );
}
