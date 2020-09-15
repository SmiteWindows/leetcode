// https://leetcode-cn.com/problems/maximum-69-number/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn maximum69_number(mut num: i32) -> i32 {
    let mut stack: Vec<i32> = vec![];
    while num > 0 {
        stack.push(num % 10);
        num /= 10;
    }
    let n = stack.len();
    let mut changed = false;
    let mut res = 0;
    for i in (0..n).rev() {
        if stack[i] == 6 && !changed {
            res = res * 10 + 9;
            changed = true;
        } else {
            res = res * 10 + stack[i];
        }
    }
    res
}
// math
#[test]
fn test1_1323() {
    assert_eq!(maximum69_number(9669), 9969);
    assert_eq!(maximum69_number(9996), 9999);
    assert_eq!(maximum69_number(9999), 9999);
}
