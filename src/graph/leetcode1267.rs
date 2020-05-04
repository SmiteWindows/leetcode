// https://leetcode.com/problems/count-servers-that-communicate/
pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// graph array
#[test]
#[ignore]
fn test1_1267() {
    assert_eq!(count_servers(vec![vec![1, 0], vec![0, 1]]), 0);
    assert_eq!(count_servers(vec![vec![1, 0], vec![1, 1]]), 3);
    assert_eq!(
        count_servers(vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1]
        ]),
        4
    );
}
