// https://leetcode-cn.com/problems/next-greater-element-iii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn next_greater_element(n: i32) -> i32 {
    let mut s = format!("{}", n).chars().collect::<Vec<_>>();
    let n = s.len();
    let mut l = n;
    for i in (0..n - 1).rev() {
        if s[i] < s[i + 1] {
            l = i;
            break;
        }
    }
    if l == n {
        return -1;
    }
    let mut max = s[l + 1];
    let mut r = l + 1;
    for i in l + 2..n {
        if s[i] > s[l] && s[i] < max {
            max = s[i];
            r = i;
        }
    }
    s.swap(l, r);
    s[l + 1..].sort_unstable();
    s.iter().collect::<String>().parse::<i32>().unwrap_or(-1)
}
// string
#[test]
fn test1_556() {
    assert_eq!(next_greater_element(12), 21);
    assert_eq!(next_greater_element(21), -1);
}
