// https://leetcode-cn.com/problems/find-common-characters/
pub fn common_chars(a: Vec<String>) -> Vec<String> {
    todo!()
}
// hash_table array
#[test]
#[ignore]
fn test1_1002() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        common_chars(vec_string!["bella", "label", "roller"]),
        vec_string!["e", "l", "l"]
    );
    assert_eq!(
        common_chars(vec_string!["cool", "lock", "cook"]),
        vec_string!["c", "o"]
    );
}
