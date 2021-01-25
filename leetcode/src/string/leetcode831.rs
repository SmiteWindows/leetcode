// https://leetcode-cn.com/problems/masking-personal-information/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn mask_pii(s: String) -> String {
    if let Some(i) = s.find('@') {
        let s = s.to_lowercase();
        format!("{}*****{}", &s[0..1], &s[i - 1..])
    } else {
        let digits = s
            .chars()
            .filter(|&c| ('0'..='9').contains(&c))
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
        mask_pii("LeetCode@LeetCode.com".to_string()),
        "l*****e@leetcode.com".to_string()
    );
    assert_eq!(
        mask_pii("AB@qq.com".to_string()),
        "a*****b@qq.com".to_string()
    );
    assert_eq!(
        mask_pii("1(234)567-890".to_string()),
        "***-***-7890".to_string()
    );
    assert_eq!(
        mask_pii("86-(10)12345678".to_string()),
        "+**-***-***-5678".to_string()
    );
}
