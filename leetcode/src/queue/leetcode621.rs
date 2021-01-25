// https://leetcode-cn.com/problems/task-scheduler/
// Runtime: 20 ms
// Memory Usage: 2.6 MB
use std::collections::HashMap;
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let n = n as usize;
    let m = tasks.len();
    let mut hm: HashMap<char, usize> = HashMap::new();
    let mut max_count = 0;
    for t in tasks {
        let count = hm.entry(t).or_default();
        *count += 1;
        max_count = max_count.max(*count);
    }
    let task_with_max_count = hm.values().filter(|&&x| x == max_count).count();
    m.max((n + 1) * (max_count - 1) + task_with_max_count) as i32
}
// array greedy queue
#[test]
fn test1_621() {
    assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
}
