// https://leetcode.com/problems/similar-string-groups/
pub fn num_similar_groups(a: Vec<String>) -> i32 {
    todo!()
}
// union_find depth_first_search graph
#[test]
#[ignore]
fn test3_839() {
    assert_eq!(
        num_similar_groups(vec![
            String::from("tars"),
            String::from("rats"),
            String::from("arts"),
            String::from("star")
        ]),
        2
    );
}
