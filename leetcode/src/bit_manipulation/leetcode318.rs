// https://leetcode-cn.com/problems/maximum-product-of-word-lengths/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::collections::HashMap;
pub fn max_product(words: Vec<String>) -> i32 {
    let mut hm: HashMap<u32, usize> = HashMap::new();
    for word in words {
        let mut mask = 0_u32;
        for c in word.bytes() {
            mask |= 1 << (c - b'a');
        }
        let size = hm.entry(mask).or_default();
        *size = word.len().max(*size);
    }
    let mut res = 0;
    for (&ka, &va) in &hm {
        for (&kb, &vb) in &hm {
            if ka & kb == 0 {
                res = res.max(va * vb);
            }
        }
    }
    res as i32
}
// bit_manipulation
#[test]
fn test1_318() {
    assert_eq!(
        max_product(vec![
            String::from("abcw"),
            String::from("baz"),
            String::from("foo"),
            String::from("bar"),
            String::from("xtfn"),
            String::from("abcdef")
        ]),
        16
    );
    assert_eq!(
        max_product(vec![
            String::from("a"),
            String::from("ab"),
            String::from("abc"),
            String::from("d"),
            String::from("cd"),
            String::from("bcd"),
            String::from("abcd")
        ]),
        4
    );
    assert_eq!(
        max_product(vec![
            String::from("a"),
            String::from("aa"),
            String::from("aaa"),
            String::from("aaaa")
        ]),
        0
    );
}
