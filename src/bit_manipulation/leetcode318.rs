// https://leetcode.com/problems/maximum-product-of-word-lengths/
pub fn max_product(words: Vec<String>) -> i32 {
    todo!()
}
// bit_manipulation
#[test]
#[ignore]
fn test1_268() {
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
