// https://leetcode.com/problems/number-of-enclaves/
pub fn num_enclaves(a: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_1020() {
    assert_eq!(
        num_enclaves(vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0]
        ]),
        3
    );
    assert_eq!(
        num_enclaves(vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0]
        ]),
        0
    );
}
