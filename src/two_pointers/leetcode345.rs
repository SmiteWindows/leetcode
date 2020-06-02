// https://leetcode.com/problems/reverse-vowels-of-a-string/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
pub fn reverse_vowels(s: String) -> String {
    let mut a = s.chars().collect::<Vec<_>>();
    let n = a.len();
    if n == 0 {
        return "".to_string();
    }
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        if is_vowel(a[l]) && is_vowel(a[r]) {
            a.swap(l, r);
            l += 1;
            r -= 1;
        } else {
            if !is_vowel(a[l]) {
                l += 1;
            }
            if !is_vowel(a[r]) {
                r -= 1;
            }
        }
    }
    a.iter().collect()
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}
// string two_pointers
#[test]
fn test1_345() {
    assert_eq!(reverse_vowels(String::from("hello")), String::from("holle"));
    assert_eq!(
        reverse_vowels(String::from("leetcode")),
        String::from("leotcede")
    );
}
