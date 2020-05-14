// https://leetcode.com/problems/reverse-only-letters/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn reverse_only_letters(s: String) -> String {
    let n = s.len();
    if n == 0 {
        return "".to_string();
    }
    let mut a: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut j = n - 1;
    while i < j {
        while i < j && !a[i].is_alphabetic() {
            i += 1;
        }
        while i < j && !a[j].is_alphabetic() {
            j -= 1;
        }
        if i < j {
            a.swap(i, j);
        }
        i += 1;
        j -= 1;
    }
    a.iter().collect()
}
// string
#[test]
fn test1_917() {
    assert_eq!(
        reverse_only_letters(String::from("ab-cd")),
        String::from("dc-ba")
    );
    assert_eq!(
        reverse_only_letters(String::from("a-bC-dEf-ghIj")),
        String::from("j-Ih-gfE-dCba")
    );
    assert_eq!(
        reverse_only_letters(String::from("Test1ng-Leet=code-Q!")),
        String::from("Qedo1ct-eeLg=ntse-T!")
    );
}
