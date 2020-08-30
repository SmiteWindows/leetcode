// https://leetcode-cn.com/problems/find-kth-bit-in-nth-binary-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::cmp::Ordering;
pub fn find_kth_bit(n: i32, k: i32) -> char {
    if find(n, k - 1) == 0 {
        '0'
    } else {
        '1'
    }
}

fn find(n: i32, k: i32) -> i32 {
    if n == 1 {
        0
    } else {
        let size = (1 << n) - 1;
        match (size / 2).cmp(&k) {
            Ordering::Equal => 1,
            Ordering::Greater => find(n - 1, k),
            Ordering::Less => 1 - find(n - 1, size - 1 - k),
        }
    }
}
// string
#[test]
fn test1_1545() {
    assert_eq!(find_kth_bit(3, 1), '0');
    assert_eq!(find_kth_bit(4, 11), '1');
    assert_eq!(find_kth_bit(1, 1), '0');
    assert_eq!(find_kth_bit(2, 3), '1');
}
