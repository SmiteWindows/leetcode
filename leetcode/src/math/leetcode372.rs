// https://leetcode-cn.com/problems/super-pow
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn super_pow(a: i32, mut b: Vec<i32>) -> i32 {
    let a = a % 1337;
    if let Some(last) = b.pop() {
        pow_mod(super_pow(a, b) % 1337, 10) * pow_mod(a, last) % 1337
    } else {
        1
    }
}

fn pow_mod(a: i32, k: i32) -> i32 {
    let mut res = 1;
    for _ in 0..k {
        res *= a;
        res %= 1337;
    }
    res
}
// math
#[test]
fn test1_372() {
    assert_eq!(super_pow(2, vec![3]), 8);
    assert_eq!(super_pow(2, vec![1, 0]), 1024);
}
