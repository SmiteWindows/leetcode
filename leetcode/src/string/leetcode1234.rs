// https://leetcode-cn.com/problems/replace-the-substring-for-balanced-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn balanced_string(s: String) -> i32 {
    let mut count = vec![0; 256];
    let s = s.bytes().collect::<Vec<_>>();
    let n = s.len();
    let k = n / 4;
    for i in 0..n {
        count[s[i] as usize] += 1;
    }
    let mut start = 0;
    let mut end = 0;
    let mut res = n;
    while end < n {
        count[s[end] as usize] -= 1;
        end += 1;
        while start < n
            && count[b'Q' as usize] <= k
            && count[b'W' as usize] <= k
            && count[b'E' as usize] <= k
            && count[b'R' as usize] <= k
        {
            res = res.min(end - start);
            count[s[start] as usize] += 1;
            start += 1;
        }
    }
    res as i32
}
// two_pointers string
#[test]
fn test2_1234() {
    assert_eq!(balanced_string("QWER".to_string()), 0);
    assert_eq!(balanced_string("QQWE".to_string()), 1);
    assert_eq!(balanced_string("QQQW".to_string()), 2);
    assert_eq!(balanced_string("QQQQ".to_string()), 3);
}
