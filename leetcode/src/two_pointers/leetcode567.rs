// https://leetcode-cn.com/problems/permutation-in-string/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn check_inclusion(s1: String, s2: String) -> bool {
    let mut c1 = [0; 26];
    let mut c2 = [0; 26];
    let n1 = s1.len();
    let n2 = s2.len();
    let s1 = s1.bytes().collect::<Vec<_>>();
    let s2 = s2.bytes().collect::<Vec<_>>();
    if n1 > n2 {
        return false;
    }
    for i in 0..n1 {
        c1[(s1[i] - b'a') as usize] += 1;
        c2[(s2[i] - b'a') as usize] += 1;
    }
    if c1 == c2 {
        return true;
    }
    for i in n1..n2 {
        c2[(s2[i] - b'a') as usize] += 1;
        c2[(s2[i - n1] - b'a') as usize] -= 1;
        if c1 == c2 {
            return true;
        }
    }
    false
}
// sliding_window two_pointers
#[test]
fn test2_567() {
    assert_eq!(
        check_inclusion("ab".to_string(), "eidbaooo".to_string()),
        true
    );
    assert_eq!(
        check_inclusion("ab".to_string(), "eidboaoo".to_string()),
        false
    );
}
