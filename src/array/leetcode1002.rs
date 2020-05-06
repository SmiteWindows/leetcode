// https://leetcode.com/problems/find-common-characters/
pub fn common_chars(a: Vec<String>) -> Vec<String> {
    todo!()
}
// hash_table array
#[test]
#[ignore]
fn test2_1002() {
    assert_eq!(
        common_chars(vec![
            String::from("bella"),
            String::from("label"),
            String::from("roller")
        ]),
        vec![String::from("e"), String::from("l"), String::from("l")]
    );
    assert_eq!(
        common_chars(vec![
            String::from("cool"),
            String::from("lock"),
            String::from("cook")
        ]),
        vec![String::from("c"), String::from("o")]
    );
}
