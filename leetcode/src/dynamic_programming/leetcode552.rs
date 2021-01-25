// https://leetcode-cn.com/problems/student-attendance-record-ii/
// Runtime: 24 ms
// Memory Usage: 4.4 MB
const MOD: i64 = 1_000_000_007;
pub fn check_record(n: i32) -> i32 {
    let n = n as usize;
    if n == 1 {
        return 3;
    }
    let mut p = vec![0; n + 1];
    let mut l = vec![0; n + 1];
    p[1] = 1;
    l[1] = 1;
    p[2] = 2;
    l[2] = 2;
    for i in 3..=n {
        p[i] = l[i - 1] + p[i - 1];
        p[i] %= MOD;
        l[i] = p[i - 1] + p[i - 2];
        l[i] %= MOD;
    }
    let mut lp = vec![0; n + 1];
    lp[0] = 1;
    for i in 1..=n {
        lp[i] = l[i] + p[i];
        lp[i] %= MOD;
    }

    let mut res: i64 = lp[n];
    for i in 0..n {
        res += (lp[i]) * (lp[n - 1 - i]);
        res %= MOD;
    }
    res as i32
}
// dynamic_programming
#[test]
fn test1_552() {
    assert_eq!(check_record(2), 8);
}
