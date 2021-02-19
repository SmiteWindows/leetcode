// https://leetcode-cn.com/problems/maximum-profit-in-job-scheduling/
// Runtime: 20 ms
// Memory Usage: 4.5 MB
use std::collections::BTreeMap;
pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut jobs = Vec::new();
    let n = start_time.len();
    for i in 0..n {
        jobs.push((start_time[i], end_time[i], profit[i]));
    }
    jobs.sort_unstable();
    let mut memo: BTreeMap<i32, i32> = BTreeMap::new();
    let mut res = 0;
    for i in (0..n).rev() {
        let mut cur = jobs[i].2;
        if let Some((_, val)) = memo.range(jobs[i].1..).next() {
            cur += val;
        }
        res = res.max(cur);
        memo.insert(jobs[i].0, res);
    }
    res
}
// sort binary_search dynamic_programming
#[test]
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
