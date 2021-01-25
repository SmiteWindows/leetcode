// https://leetcode-cn.com/problems/peak-index-in-a-mountain-array/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = a.len() - 1;
    while l < r {
        let m = l + (r - l) / 2;
        if a[m] < a[m + 1] {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l as i32
}
// binary_search
#[test]
fn test1_852() {
    assert_eq!(peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
}
