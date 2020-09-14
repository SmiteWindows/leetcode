// https://leetcode-cn.com/problems/minimum-swaps-to-make-strings-equal/
pub fn minimum_swap(s1: String, s2: String) -> i32 {
    todo!()
}
// greedy string
#[test]
#[ignore]
fn test1_1247() {
    assert_eq!(minimum_swap("xx".to_string(), "yy".to_string()), 1);
    assert_eq!(minimum_swap("xy".to_string(), "yx".to_string()), 2);
    assert_eq!(minimum_swap("xx".to_string(), "xy".to_string()), -1);
    assert_eq!(
        minimum_swap("xxyyxyxyxx".to_string(), "xyyxyxxxyx".to_string()),
        4
    );
}
