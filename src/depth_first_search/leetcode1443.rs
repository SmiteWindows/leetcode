// https://leetcode.com/problems/minimum-time-to-collect-all-apples-in-a-tree/
pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
    todo!()
}
// tree depth_first_search
#[test]
#[ignore]
fn test2_1443() {
    assert_eq!(
        min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            vec![false, false, true, false, true, true, false]
        ),
        8
    );
    assert_eq!(
        min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            vec![false, false, true, false, false, true, false]
        ),
        6
    );
    assert_eq!(
        min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            vec![false, false, false, false, false, false, false]
        ),
        0
    );
}
