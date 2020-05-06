// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/
pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_1282() {
    assert_eq!(
        group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
        vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]
    );
    assert_eq!(
        group_the_people(vec![2, 1, 3, 3, 3, 2]),
        vec![vec![1], vec![0, 5], vec![2, 3, 4]]
    );
}
