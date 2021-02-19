// https://leetcode-cn.com/problems/remove-duplicate-letters/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn remove_duplicate_letters(s: String) -> String {
    let mut stack = Vec::new();
    let mut left = vec![0; 26];
    for b in s.bytes() {
        left[(b - b'a') as usize] += 1;
    }
    let mut visited = vec![false; 26];
    for b in s.bytes() {
        left[(b - b'a') as usize] -= 1;
        if !visited[(b - b'a') as usize] {
            visited[(b - b'a') as usize] = true;
            while let Some(&top) = stack.last() {
                if top > b && left[(top - b'a') as usize] > 0 {
                    visited[(top - b'a') as usize] = false;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(b);
        }
    }
    stack.into_iter().map(|b| b as char).collect()
}
// stack greedy
#[test]
fn test1_316() {
    assert_eq!(
        remove_duplicate_letters("bcabc".to_string()),
        "abc".to_string()
    );
    assert_eq!(
        remove_duplicate_letters("cbacdcbc".to_string()),
        "acdb".to_string()
    );
}
