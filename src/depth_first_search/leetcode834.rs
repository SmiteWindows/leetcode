// https://leetcode.com/problems/sum-of-distances-in-tree/
pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// tree depth_first_search
#[test]
fn test2_834() {
    assert_eq!(
        sum_of_distances_in_tree(
            6,
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]]
        ),
        vec![8, 12, 6, 10, 10, 10]
    );
}
