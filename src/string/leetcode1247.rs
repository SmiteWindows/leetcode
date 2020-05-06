// https://leetcode.com/problems/minimum-swaps-to-make-strings-equal/
pub fn minimum_swap(s1: String, s2: String) -> i32 {
    todo!()
}
// greedy string
#[test]
#[ignore]
fn test2_1247() {
    assert_eq!(minimum_swap(String::from("xx"), String::from("yy")), 1);
    assert_eq!(minimum_swap(String::from("xy"), String::from("yx")), 2);
    assert_eq!(minimum_swap(String::from("xx"), String::from("xy")), -1);
    assert_eq!(
        minimum_swap(String::from("xxyyxyxyxx"), String::from("xyyxyxxxyx")),
        4
    );
}
