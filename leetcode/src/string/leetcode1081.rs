// https://leetcode-cn.com/problems/smallest-subsequence-of-distinct-characters/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn smallest_subsequence(text: String) -> String {
    let mut stack = vec![];
    let mut left = vec![0; 26];
    for b in text.bytes() {
        left[(b - b'a') as usize] += 1;
    }
    let mut visited = vec![false; 26];
    for b in text.bytes() {
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
// string
#[test]
fn test1_1081() {
    assert_eq!(
        smallest_subsequence("cdadabcc".to_string()),
        "adbc".to_string()
    );
    assert_eq!(smallest_subsequence("abcd".to_string()), "abcd".to_string());
    assert_eq!(
        smallest_subsequence("ecbacba".to_string()),
        "eacb".to_string()
    );
    assert_eq!(
        smallest_subsequence("leetcode".to_string()),
        "letcod".to_string()
    );
}
