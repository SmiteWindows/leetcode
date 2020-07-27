// https://leetcode.com/problems/reverse-vowels-of-a-string/
// Runtime: 0 ms
// Memory Usage: 2.7 MB
pub fn reverse_vowels(s: String) -> String {
    let mut a = s.chars().collect::<Vec<_>>();
    let n = a.len();
    if n == 0 {
        return "".to_string();
    }
    let mut left = 0;
    let mut right = n - 1;
    while left < right {
        if is_vowel(a[left]) && is_vowel(a[right]) {
            a.swap(left, right);
            left += 1;
            right -= 1;
        } else {
            if !is_vowel(a[left]) {
                left += 1;
            }
            if !is_vowel(a[right]) {
                right -= 1;
            }
        }
    }
    a.iter().collect()
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}
// string two_pointers
#[test]
fn test2_345() {
    assert_eq!(reverse_vowels(String::from("hello")), String::from("holle"));
    assert_eq!(
        reverse_vowels(String::from("leetcode")),
        String::from("leotcede")
    );
}
