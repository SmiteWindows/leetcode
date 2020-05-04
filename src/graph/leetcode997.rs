// https://leetcode.com/problems/find-the-town-judge/
pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// graph
#[test]
#[ignore]
fn test1_997() {
    assert_eq!(find_judge(2, vec![vec![1, 2]]), 2);
    assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]), -1);
    assert_eq!(find_judge(3, vec![vec![1, 2], vec![2, 3]]), -1);
    assert_eq!(
        find_judge(
            4,
            vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]]
        ),
        3
    );
}
