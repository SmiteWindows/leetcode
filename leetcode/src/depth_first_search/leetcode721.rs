// https://leetcode-cn.com/problems/accounts-merge/
pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    todo!()
}
// depth_first_search union_find
#[test]
#[ignore]
fn test2_721() {
    use leetcode_prelude::{assert_eq_sorted, vec2_string};
    assert_eq_sorted!(
        accounts_merge(vec2_string![
            ["John", "johnsmith@mail.com", "john00@mail.com"],
            ["John", "johnnybravo@mail.com"],
            ["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            ["Mary", "mary@mail.com"]
        ]),
        vec2_string![
            [
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com"
            ],
            ["John", "johnnybravo@mail.com"],
            ["Mary", "mary@mail.com"]
        ]
    );
}
