// https://leetcode.com/problems/contain-virus/
pub fn contain_virus(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_749() {
    assert_eq!(
        contain_virus(vec![
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0]
        ]),
        10
    );
    assert_eq!(
        contain_virus(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        4
    );
    assert_eq!(
        contain_virus(vec![
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0]
        ]),
        13
    );
}
