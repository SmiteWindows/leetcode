// https://leetcode-cn.com/problems/most-stones-removed-with-same-row-or-column/
pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// union_find depth_first_search
#[test]
#[ignore]
fn test2_947() {
    assert_eq!(
        remove_stones(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2]
        ]),
        5
    );
    assert_eq!(
        remove_stones(vec![
            vec![0, 0],
            vec![0, 2],
            vec![1, 1],
            vec![2, 0],
            vec![2, 2]
        ]),
        3
    );
    assert_eq!(remove_stones(vec![vec![0, 0]]), 0);
}
