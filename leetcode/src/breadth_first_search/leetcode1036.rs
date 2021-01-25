// https://leetcode-cn.com/problems/escape-a-large-maze/
pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1036() {
    use leetcode_prelude::vec2;
    assert_eq!(
        is_escape_possible(vec2![[0, 1], [1, 0]], vec![0, 0], vec![0, 2]),
        false
    );
    assert_eq!(
        is_escape_possible(vec2![[]], vec![0, 0], vec![999999, 999999]),
        true
    );
}
