// https://leetcode.com/problems/path-with-maximum-gold/
pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// backtracking
#[test]
#[ignore]
fn test1_1219() {
    assert_eq!(
        get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]),
        24
    );
    assert_eq!(
        get_maximum_gold(vec![
            vec![1, 0, 7],
            vec![2, 0, 6],
            vec![3, 4, 5],
            vec![0, 3, 0],
            vec![9, 0, 20]
        ]),
        28
    );
}
