// https://leetcode-cn.com/problems/reverse-only-letters/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn reverse_only_letters(s: String) -> String {
    let n = s.len();
    if n == 0 {
        return "".to_string();
    }
    let mut res = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut j = n - 1;
    while i < j {
        while i < j && !res[i].is_alphabetic() {
            i += 1;
        }
        while i < j && !res[j].is_alphabetic() {
            j -= 1;
        }
        if i < j {
            res.swap(i, j);
        }
        i += 1;
        j -= 1;
    }
    res.iter().collect()
}
// string
#[test]
fn test1_917() {
    assert_eq!(
        reverse_only_letters("ab-cd".to_string()),
        "dc-ba".to_string()
    );
    assert_eq!(
        reverse_only_letters("a-bC-dEf-ghIj".to_string()),
        "j-Ih-gfE-dCba".to_string()
    );
    assert_eq!(
        reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
        "Qedo1ct-eeLg=ntse-T!".to_string()
    );
}
