// https://leetcode-cn.com/problems/letter-case-permutation/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn letter_case_permutation(s: String) -> Vec<String> {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut res = Vec::new();
    let mut t = "".to_string();
    permutation(&s, n, 0, &mut t, &mut res);
    res
}

fn permutation(s: &[char], n: usize, i: usize, t: &mut String, res: &mut Vec<String>) {
    if i == n {
        res.push(t.clone());
    } else if s[i].is_alphabetic() {
        let lower: char = s[i].to_ascii_lowercase();
        let upper: char = s[i].to_ascii_uppercase();
        t.push(lower);
        permutation(s, n, i + 1, t, res);
        t.pop();
        t.push(upper);
        permutation(s, n, i + 1, t, res);
        t.pop();
    } else {
        t.push(s[i]);
        permutation(s, n, i + 1, t, res);
        t.pop();
    }
}
// backtracking bit_manipulation
#[test]
fn test1_784() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        letter_case_permutation("a1b2".to_string()),
        vec_string!["a1b2", "a1B2", "A1b2", "A1B2"]
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
