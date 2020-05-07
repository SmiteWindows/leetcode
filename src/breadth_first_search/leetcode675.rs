// https://leetcode.com/problems/cut-off-trees-for-golf-event/
pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_675() {
    assert_eq!(
        cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5]]),
        6
    );
    assert_eq!(
        cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 0], vec![7, 6, 5]]),
        -1
    );
    assert_eq!(
        cut_off_tree(vec![vec![2, 3, 4], vec![0, 0, 5], vec![8, 7, 6]]),
        6
    );
}
