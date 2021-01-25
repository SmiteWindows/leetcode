// https://leetcode-cn.com/problems/similar-string-groups/
pub fn num_similar_groups(a: Vec<String>) -> i32 {
    todo!()
}
// union_find depth_first_search graph
#[test]
#[ignore]
fn test2_839() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        num_similar_groups(vec_string!["tars", "rats", "arts", "star"]),
        2
    );
}
