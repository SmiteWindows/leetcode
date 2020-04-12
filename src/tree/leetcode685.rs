// https://leetcode.com/problems/redundant-connection-ii/
pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// tree graph union_find depth_first_search
#[test]
#[ignore]
fn test1_685() {
    assert_eq!(
        find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
        vec![2, 3]
    );
    assert_eq!(
        find_redundant_directed_connection(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 1],
            vec![1, 5]
        ]),
        vec![4, 1]
    );
}
