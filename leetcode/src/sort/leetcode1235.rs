// https://leetcode-cn.com/problems/maximum-profit-in-job-scheduling/
pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    todo!()
}
// sort binary_search dynamic_programming
#[test]
#[ignore]
fn test1_1235() {
    assert_eq!(
        job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
        120
    );
    assert_eq!(
        job_scheduling(
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 20, 100, 70, 60]
        ),
        150
    );
    assert_eq!(
        job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
        6
    );
}
