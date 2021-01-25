// https://leetcode-cn.com/problems/prime-number-of-set-bits-in-binary-representation/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
    let mut count = 0;
    for i in l..=r {
        match i.count_ones() {
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => count += 1,
            _ => {}
        }
    }
    count
}
// bit_manipulation
#[test]
fn test1_762() {
    assert_eq!(count_prime_set_bits(6, 10), 4);
    assert_eq!(count_prime_set_bits(10, 15), 5);
}
