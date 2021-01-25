// https://leetcode-cn.com/problems/sum-of-distances-in-tree/
pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// tree depth_first_search
#[test]
#[ignore]
fn test1_834() {
    use leetcode_prelude::vec2;
    assert_eq!(
        sum_of_distances_in_tree(6, vec2![[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]]),
        vec![8, 12, 6, 10, 10, 10]
    );
}
