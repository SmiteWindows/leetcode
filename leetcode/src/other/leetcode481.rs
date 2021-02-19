// https://leetcode-cn.com/problems/magical-string/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
pub fn magical_string(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n <= 3 {
        return 1;
    }
    let n = n as usize;
    let mut a = vec![1, 2, 2];
    let mut i = 2;
    let mut x = 1;
    let mut res = 1;
    loop {
        for _ in 0..a[i] {
            if x == 1 {
                res += 1;
            }
            a.push(x);
            if a.len() >= n {
                return res;
            }
        }
        x = 3 - x;
        i += 1;
    }
}
#[test]
fn test481() {
    assert_eq!(magical_string(6), 3);
}
