// https://leetcode-cn.com/problems/flip-string-to-monotone-increasing/
// Runtime: 0 ms
// Memory Usage: 2.4 MB
pub fn min_flips_mono_incr(s: String) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<char>>();
    let mut left = vec![0; n];
    let mut ones = 0;
    let mut right = vec![0; n];
    let mut zeros = 0;
    for i in 0..n {
        left[i] = ones;
        if s[i] == '1' {
            ones += 1;
        }
    }
    for i in (0..n).rev() {
        right[i] = zeros;
        if s[i] == '0' {
            zeros += 1;
        }
    }
    let mut res = i32::MAX;
    for i in 0..n {
        res = res.min(left[i] + right[i]);
    }
    res
}
// array
#[test]
fn test1_926() {
    assert_eq!(min_flips_mono_incr("00110".to_string()), 1);
    assert_eq!(min_flips_mono_incr("010110".to_string()), 2);
    assert_eq!(min_flips_mono_incr("00011000".to_string()), 2);
}
