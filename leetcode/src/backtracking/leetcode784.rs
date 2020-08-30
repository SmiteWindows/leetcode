// https://leetcode-cn.com/problems/letter-case-permutation/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn letter_case_permutation(s: String) -> Vec<String> {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut res = vec![];
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
    assert_eq!(
        letter_case_permutation(String::from("a1b2")),
        vec![
            String::from("a1b2"),
            String::from("a1B2"),
            String::from("A1b2"),
            String::from("A1B2")
        ]
    );
    assert_eq!(
        letter_case_permutation(String::from("3z4")),
        vec![String::from("3z4"), String::from("3Z4")]
    );
    assert_eq!(
        letter_case_permutation(String::from("12345")),
        vec![String::from("12345")]
    );
}
