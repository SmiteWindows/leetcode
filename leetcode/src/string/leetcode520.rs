// https://leetcode.com/problems/detect-capital/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn detect_capital_use(word: String) -> bool {
    let n = word.len();
    if n <= 1 {
        return true;
    }
    let word = word.chars().collect::<Vec<_>>();
    let first_is_lowercase = word[0].is_lowercase();
    if first_is_lowercase {
        for (i, w) in word.iter().enumerate().skip(1) {
            if w.is_uppercase() {
                return false;
            }
        }
    } else {
        let mut prev = None;
        for (i, w) in word.iter().enumerate().skip(1) {
            if let Some(prev_case) = prev {
                if prev_case != w.is_uppercase() {
                    return false;
                }
            } else {
                prev = Some(w.is_uppercase());
            }
        }
    }
    true
}
// string
#[test]
fn test1_520() {
    assert_eq!(detect_capital_use(String::from("USA")), true);
    assert_eq!(detect_capital_use(String::from("FlaG")), false);
}
