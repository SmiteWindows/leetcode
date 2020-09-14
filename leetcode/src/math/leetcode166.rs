// https://leetcode-cn.com/problems/fraction-to-recurring-decimal/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
use std::{collections::HashMap, iter::FromIterator};
pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    if numerator == 0 {
        return "0".to_string();
    }
    let mut res = vec![];
    if (numerator < 0) ^ (denominator < 0) {
        res.push('-');
    }
    let n = (numerator as i64).abs();
    let d = (denominator as i64).abs();
    res.append(&mut format!("{}", n / d).chars().collect());
    let mut r = n % d;
    if r == 0 {
        return String::from_iter(res);
    }
    res.push('.');
    let mut hs = HashMap::new();
    while r != 0 {
        if let Some(index) = hs.insert(r, res.len()) {
            res.insert(index, '(');
            res.push(')');
            break;
        }
        r *= 10;
        res.append(&mut format!("{}", r / d).chars().collect());
        r %= d;
    }
    String::from_iter(res)
}
// math hash_table
#[test]
fn test1_166() {
    assert_eq!(fraction_to_decimal(1, 2), "0.5".to_string());
    assert_eq!(fraction_to_decimal(2, 1), "2".to_string());
    assert_eq!(fraction_to_decimal(2, 3), "0.(6)".to_string());
}
