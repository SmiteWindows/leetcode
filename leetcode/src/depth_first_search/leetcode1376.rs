// https://leetcode-cn.com/problems/time-needed-to-inform-all-employees/
// Runtime: 40 ms
// Memory Usage: 3.3 MB
pub fn num_of_minutes(
    n: i32,
    head_id: i32,
    mut manager: Vec<i32>,
    mut inform_time: Vec<i32>,
) -> i32 {
    let n = n as usize;
    let mut res = 0;
    for i in 0..n {
        res = res.max(dfs(i, &mut manager, &mut inform_time));
    }
    res
}

fn dfs(i: usize, manager: &mut Vec<i32>, inform_time: &mut Vec<i32>) -> i32 {
    if manager[i] != -1 {
        inform_time[i] += dfs(manager[i] as usize, manager, inform_time);
        manager[i] = -1;
    }
    inform_time[i]
}
// depth_first_search
#[test]
fn test1_1376() {
    assert_eq!(num_of_minutes(1, 0, vec![-1], vec![0]), 0);
    assert_eq!(
        num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0]),
        1
    );
    assert_eq!(
        num_of_minutes(7, 6, vec![1, 2, 3, 4, 5, 6, -1], vec![0, 6, 5, 4, 3, 2, 1]),
        21
    );
    assert_eq!(
        num_of_minutes(
            15,
            0,
            vec![-1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6],
            vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        ),
        3
    );
    assert_eq!(
        num_of_minutes(4, 2, vec![3, 3, -1, 2], vec![0, 0, 162, 914]),
        1076
    );
}
