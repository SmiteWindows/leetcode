// https://leetcode.com/problems/perfect-number/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn check_perfect_number(num: i32) -> bool {
    if num == 1 {
        return false;
    }
    let mut i = 2;
    let mut sum = 1;
    while i * i <= num {
        if num % i == 0 {
            sum += i;
            sum += num / i;
        }
        i += 1;
    }
    sum == num
}
// math
#[test]
fn test1_507() {
    assert_eq!(check_perfect_number(28), true);
}
