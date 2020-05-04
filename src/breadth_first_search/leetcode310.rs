// https://leetcode.com/problems/minimum-height-trees/
pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// graph breadth_first_search
#[test]
#[ignore]
fn test2_310() {
    assert_eq!(
        find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
        vec![1]
    );
    assert_eq!(
        find_min_height_trees(
            6,
            vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 4]]
        ),
        vec![3, 4]
    );
}
