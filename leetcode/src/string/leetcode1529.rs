// https://leetcode.com/problems/bulb-switcher-iv/
// Runtime: 4 ms
// Memory Usage: 2.8 MB
pub fn min_flips(target: String) -> i32 {
    let n = target.len();
    let s = target.chars().collect::<Vec<_>>();
    let mut res = 0;
    let mut i = 0;
    let mut prev = '0';
    loop {
        while i < n && s[i] == prev {
            i += 1;
        }
        if i == n {
            break;
        }
        prev = s[i];
        res += 1;
    }
    res
}
// string
#[test]
fn test1_1529() {
    assert_eq!(min_flips("10111".to_string()), 3);
    assert_eq!(min_flips("101".to_string()), 3);
    assert_eq!(min_flips("00000".to_string()), 0);
    assert_eq!(min_flips("001011101".to_string()), 5);
}
