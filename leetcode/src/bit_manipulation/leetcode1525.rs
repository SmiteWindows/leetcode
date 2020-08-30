// https://leetcode-cn.com/problems/number-of-good-ways-to-split-a-string/
// Runtime: 8 ms
// Memory Usage: 2.1 MB
pub fn num_splits(s: String) -> i32 {
    let mut total = vec![0; 26];
    for b in s.bytes() {
        total[(b - b'a') as usize] += 1;
    }
    let mut left = vec![0; 26];
    let mut right = total.to_vec();
    let mut res = 0;
    for b in s.bytes() {
        left[(b - b'a') as usize] += 1;
        right[(b - b'a') as usize] -= 1;
        let mut diff = 0;
        for i in 0..26 {
            if left[i] > 0 {
                diff += 1;
            }
            if right[i] > 0 {
                diff -= 1;
            }
        }
        if diff == 0 {
            res += 1;
        }
    }
    res
}
// string bit_manipulation
#[test]
fn test2_1525() {
    assert_eq!(num_splits("aacaba".to_string()), 2);
    assert_eq!(num_splits("abcd".to_string()), 1);
    assert_eq!(num_splits("aaaaa".to_string()), 4);
    assert_eq!(num_splits("acbadbaada".to_string()), 2);
}
