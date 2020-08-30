// https://leetcode-cn.com/problems/number-of-steps-to-reduce-a-number-to-zero/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn number_of_steps(num: i32) -> i32 {
    let mut num = num;
    let mut res = 0;
    while num != 0 {
        if num & 1 == 1 {
            num -= 1;
        } else {
            num >>= 1;
        }
        res += 1;
    }
    res
}
// bit_manipulation
#[test]
fn test1_1342() {
    assert_eq!(number_of_steps(14), 6);
    assert_eq!(number_of_steps(8), 4);
    assert_eq!(number_of_steps(123), 12);
}
