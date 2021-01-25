// https://leetcode-cn.com/problems/letter-case-permutation/
// Runtime: 4 ms
// Memory Usage: 2.4 MB
pub fn letter_case_permutation(s: String) -> Vec<String> {
    let mut b = 0;
    for c in s.chars() {
        if c.is_alphabetic() {
            b += 1;
        }
    }
    let mut res = vec![];
    for bits in 0..1 << b {
        let mut b = 0;
        let mut word = "".to_string();
        for letter in s.chars() {
            if letter.is_alphabetic() {
                if ((bits >> b) & 1) == 1 {
                    word.push(letter.to_ascii_uppercase());
                    b += 1;
                } else {
                    word.push(letter.to_ascii_lowercase());
                    b += 1;
                }
            } else {
                word.push(letter);
            }
        }
        res.push(word.to_string());
    }
    res
}
// backtracking bit_manipulation
#[test]
fn test2_784() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        letter_case_permutation("a1b2".to_string()),
        vec_string!["a1b2", "A1b2", "a1B2", "A1B2"]
    );
    assert_eq!(
        letter_case_permutation("3z4".to_string()),
        vec_string!["3z4", "3Z4"]
    );
    assert_eq!(
        letter_case_permutation("12345".to_string()),
        vec_string!["12345"]
    );
}
