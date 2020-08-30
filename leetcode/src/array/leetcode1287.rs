// https://leetcode-cn.com/problems/element-appearing-more-than-25-in-sorted-array/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let t = n / 4;
    for i in 0..n - t {
        if arr[i] == arr[i + t] {
            return arr[i];
        }
    }
    0
}
// array
#[test]
fn test1_1287() {
    assert_eq!(find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]), 6);
}
