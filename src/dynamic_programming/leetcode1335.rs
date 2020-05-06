// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/
pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1335() {
    assert_eq!(min_difficulty(vec![6, 5, 4, 3, 2, 1], 2), 7);
    assert_eq!(min_difficulty(vec![9, 9, 9], 4), -1);
    assert_eq!(min_difficulty(vec![1, 1, 1], 3), 3);
    assert_eq!(min_difficulty(vec![7, 1, 7, 1, 7, 1], 3), 15);
    assert_eq!(
        min_difficulty(vec![11, 111, 22, 222, 33, 333, 44, 444], 6),
        843
    );
}
