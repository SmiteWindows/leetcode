// https://leetcode.com/problems/kth-missing-positive-number/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    let mut k = k;
    let mut x = 1;
    let mut i = 0;
    let n = arr.len();
    loop {
        if i < n && x == arr[i] {
            i += 1;
        } else {
            k -= 1;
        }
        if k == 0 {
            break;
        }
        x += 1;
    }
    x
}
// array hash_table
#[test]
fn test1_1539() {
    assert_eq!(find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
    assert_eq!(find_kth_positive(vec![1, 2, 3, 4], 2), 6);
}
