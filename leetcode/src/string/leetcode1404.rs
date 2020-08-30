// https://leetcode-cn.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn num_steps(s: String) -> i32 {
    let mut carry = 0;
    let mut res = 0;
    let n = s.len();
    let s = s.bytes().collect::<Vec<_>>();
    for si in s.iter().skip(1).rev() {
        res += 1;
        if si - b'0' + carry == 1 {
            carry = 1;
            res += 1;
        }
    }
    res + carry as i32
}
// string bit_manipulation
#[test]
fn test2_1404() {
    assert_eq!(num_steps(String::from("1101")), 6);
    assert_eq!(num_steps(String::from("10")), 1);
    assert_eq!(num_steps(String::from("1")), 0);
}
