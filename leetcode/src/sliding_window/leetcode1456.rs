// https://leetcode-cn.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/
// Runtime: 8 ms
// Memory Usage: 2.7 MB
pub fn max_vowels(s: String, k: i32) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let k = k as usize;
    let mut cur = 0;
    let mut res = 0;
    for i in 0..n {
        if is_vowel(s[i]) {
            cur += 1;
        }
        if i >= k && is_vowel(s[i - k]) {
            cur -= 1;
        }
        res = res.max(cur);
    }
    res
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}
// string sliding_window
#[test]
fn test2_1456() {
    assert_eq!(max_vowels("abciiidef".to_string(), 3), 3);
    assert_eq!(max_vowels("aeiou".to_string(), 2), 2);
    assert_eq!(max_vowels("leetcode".to_string(), 3), 2);
    assert_eq!(max_vowels("rhythms".to_string(), 4), 0);
    assert_eq!(max_vowels("tryhard".to_string(), 4), 1);
}
