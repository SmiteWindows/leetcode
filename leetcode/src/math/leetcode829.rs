// https://leetcode-cn.com/problems/consecutive-numbers-sum/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn consecutive_numbers_sum(n: i32) -> i32 {
    let mut i = 2;
    let mut res = 1;
    while i * i < 2 * n {
        if (n - i * (i - 1) / 2) % i == 0 {
            res += 1;
        }
        i += 1;
    }
    res
}
// math
#[test]
fn test1_829() {
    assert_eq!(consecutive_numbers_sum(5), 2);
    assert_eq!(consecutive_numbers_sum(9), 3);
    assert_eq!(consecutive_numbers_sum(15), 4);
}
