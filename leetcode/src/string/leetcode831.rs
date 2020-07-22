// https://leetcode.com/problems/masking-personal-information/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn mask_pii(s: String) -> String {
    if let Some(i) = s.find('@') {
        let s = s.to_lowercase();
        format!("{}*****{}", &s[0..1], &s[i - 1..])
    } else {
        let digits = s
            .chars()
            .filter(|&c| c >= '0' && c <= '9')
            .collect::<String>();
        let n = digits.len();
        match digits.len() {
            13 => format!("+***-***-***-{}", &digits[n - 4..]),
            12 => format!("+**-***-***-{}", &digits[n - 4..]),
            11 => format!("+*-***-***-{}", &digits[n - 4..]),
            _ => format!("***-***-{}", &digits[n - 4..]),
        }
    }
}
// string
#[test]
fn test1_831() {
    assert_eq!(
        mask_pii(String::from("LeetCode@LeetCode.com")),
        String::from("l*****e@leetcode.com")
    );
    assert_eq!(
        mask_pii(String::from("AB@qq.com")),
        String::from("a*****b@qq.com")
    );
    assert_eq!(
        mask_pii(String::from("1(234)567-890")),
        String::from("***-***-7890")
    );
    assert_eq!(
        mask_pii(String::from("86-(10)12345678")),
        String::from("+**-***-***-5678")
    );
}
