// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/
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
    if let 'a' | 'e' | 'i' | 'o' | 'u' = c {
        true
    } else {
        false
    }
}
// string sliding_window
#[test]
fn test2_1456() {
    assert_eq!(max_vowels(String::from("abciiidef"), 3), 3);
    assert_eq!(max_vowels(String::from("aeiou"), 2), 2);
    assert_eq!(max_vowels(String::from("leetcode"), 3), 2);
    assert_eq!(max_vowels(String::from("rhythms"), 4), 0);
    assert_eq!(max_vowels(String::from("tryhard"), 4), 1);
}
