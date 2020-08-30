// https://leetcode-cn.com/problems/closest-divisors/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn closest_divisors(num: i32) -> Vec<i32> {
    for i in (0..=((num + 2) as f64).sqrt() as i32).rev() {
        if (num + 1) % i == 0 {
            return vec![(num + 1) / i, i];
        }
        if (num + 2) % i == 0 {
            return vec![(num + 2) / i, i];
        }
    }
    vec![]
}
// math
#[test]
fn test1_1362() {
    assert_eq!(closest_divisors(8), vec![3, 3]);
    // assert_eq!(closest_divisors(123), vec![5, 25]);
    assert_eq!(closest_divisors(123), vec![25, 5]);
    assert_eq!(closest_divisors(999), vec![40, 25]);
}
