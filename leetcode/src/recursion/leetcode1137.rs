// https://leetcode-cn.com/problems/n-th-tribonacci-number/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn tribonacci(n: i32) -> i32 {
    let n = n as usize;
    let mut a: Vec<i32> = vec![0; 38];
    a[0] = 0;
    a[1] = 1;
    a[2] = 1;
    for i in 3..=n {
        a[i] = a[i - 1] + a[i - 2] + a[i - 3];
    }
    a[n]
}
// recursion
#[test]
fn test1_1137() {
    assert_eq!(tribonacci(4), 4);
    assert_eq!(tribonacci(25), 1389537);
}
