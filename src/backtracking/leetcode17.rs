// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
use std::collections::HashMap;
pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let digits = digits.chars().collect::<Vec<_>>();
    let hm = [
        ('2', "abc"),
        ('3', "def"),
        ('4', "ghi"),
        ('5', "jkl"),
        ('6', "mno"),
        ('7', "pqrs"),
        ('8', "tuv"),
        ('9', "wxyz"),
    ]
    .iter()
    .map(|(d, v)| (*d, v.chars().collect::<Vec<_>>()))
    .collect::<HashMap<_, _>>();
    let mut s = vec![];
    let mut res = vec![];
    dfs(&hm, &digits, &mut s, &mut res, 0);
    res
}

fn dfs(
    hm: &HashMap<char, Vec<char>>,
    digits: &[char],
    s: &mut Vec<char>,
    res: &mut Vec<String>,
    index: usize,
) {
    if index == digits.len() {
        res.push(s.iter().collect());
    } else {
        let d = digits[index];
        for &c in hm[&d].iter() {
            s.push(c);
            dfs(hm, digits, s, res, index + 1);
            s.pop();
        }
    }
}
// string backtracking
#[test]
fn test1_17() {
    assert_eq!(
        letter_combinations(String::from("23")),
        vec![
            String::from("ad"),
            String::from("ae"),
            String::from("af"),
            String::from("bd"),
            String::from("be"),
            String::from("bf"),
            String::from("cd"),
            String::from("ce"),
            String::from("cf")
        ]
    );
}
