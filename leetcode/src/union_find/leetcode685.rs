// https://leetcode-cn.com/problems/redundant-connection-ii/
pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// tree graph union_find depth_first_search
#[test]
#[ignore]
fn test2_685() {
    use leetcode_prelude::vec2;
    assert_eq!(
        find_redundant_directed_connection(vec2![[1, 2], [1, 3], [2, 3]]),
        vec![2, 3]
    );
    assert_eq!(
        find_redundant_directed_connection(vec2![[1, 2], [2, 3], [3, 4], [4, 1], [1, 5]]),
        vec![4, 1]
    );
}
