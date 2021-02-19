// https://leetcode-cn.com/problems/find-the-shortest-superstring/
pub fn shortest_superstring(a: Vec<String>) -> String {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_943() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        shortest_superstring(vec_string!["alex", "loves", "leetcode"]),
        "alexlovesleetcode".to_string()
    );
    assert_eq!(
        shortest_superstring(vec_string!["catg", "ctaagt", "gcta", "ttca", "atgcatc"]),
        "gctaagttcatgcatc".to_string()
    );
}
