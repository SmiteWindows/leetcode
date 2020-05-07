// https://leetcode.com/problems/reverse-only-letters/
pub fn reverse_only_letters(s: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
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
