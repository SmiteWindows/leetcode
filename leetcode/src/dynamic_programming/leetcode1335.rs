// https://leetcode-cn.com/problems/minimum-difficulty-of-a-job-schedule/
// Runtime: 36 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
    let n = job_difficulty.len();
    let d = d as usize;
    if d > n {
        return -1;
    }
    let mut memo = HashMap::new();
    dp(0, d, &mut memo, &job_difficulty, n)
}

fn dp(
    start: usize,
    d: usize,
    memo: &mut HashMap<(usize, usize), i32>,
    jobs: &[i32],
    n: usize,
) -> i32 {
    if let Some(&res) = memo.get(&(start, d)) {
        return res;
    }
    let res = if d == 1 {
        *jobs[start..n].iter().max().unwrap()
    } else if start + d == n {
        jobs[start..start + d].iter().sum()
    } else {
        let mut min = i32::MAX;
        let mut max = 0;
        for i in start..=(n - d) {
            max = max.max(jobs[i]);
            min = min.min(max + dp(i + 1, d - 1, memo, jobs, n));
        }
        min
    };
    memo.insert((start, d), res);
    res
}
// dynamic_programming
#[test]
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
