// https://leetcode.com/problems/number-of-nodes-in-the-sub-tree-with-the-same-label/
pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
    todo!()
}
// breadth_first_search depth_first_search
#[test]
#[ignore]
fn test1_1519() {
    assert_eq!(
        count_sub_trees(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            "abaedcd".to_string()
        ),
        vec![2, 1, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        count_sub_trees(
            4,
            vec![vec![0, 1], vec![1, 2], vec![0, 3]],
            "bbbb".to_string()
        ),
        vec![4, 2, 1, 1]
    );
    assert_eq!(
        count_sub_trees(
            5,
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![0, 4]],
            "aabab".to_string()
        ),
        vec![3, 2, 1, 1, 1]
    );
    assert_eq!(
        count_sub_trees(
            6,
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![3, 4], vec![4, 5]],
            "cbabaa".to_string()
        ),
        vec![1, 2, 1, 1, 2, 1]
    );
    assert_eq!(
        count_sub_trees(
            7,
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6]
            ],
            "aaabaaa".to_string()
        ),
        vec![6, 5, 4, 1, 3, 2, 1]
    );
}
