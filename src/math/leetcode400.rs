// https://leetcode.com/problems/nth-digit/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn find_nth_digit(n: i32) -> i32 {
    let mut size = 1;
    let mut start = 1;
    let mut count = 9;
    let mut n = n as usize;
    while n > size * count {
        n -= size * count;
        size += 1;
        start *= 10;
        count *= 10;
    }
    let x = start + (n - 1) / size;
    let v = format!("{}", x)
        .bytes()
        .map(|b| (b - b'0') as i32)
        .collect::<Vec<_>>();
    v[(n - 1) % size]
}
// math
#[test]
fn test1_400() {
    assert_eq!(find_nth_digit(3), 3);
    assert_eq!(find_nth_digit(11), 0);
}
