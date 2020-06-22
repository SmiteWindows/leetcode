// https://leetcode.com/problems/task-scheduler/
// Runtime: 24 ms
// Memory Usage: 2.7 MB
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    use std::collections::HashMap;
    let n = n as usize;
    let m = tasks.len();
    let mut hm: HashMap<char, usize> = HashMap::new();
    let mut max = 0;
    for t in tasks {
        let count = hm.entry(t).or_default();
        *count += 1;
        max = max.max(*count);
    }
    let v = hm.iter().fold(
        0,
        |sum, (_, &count)| if count == max { sum + 1 } else { sum },
    );
    m.max((n + 1) * (max - 1) + v) as i32
}
// array greedy queue
#[test]
fn test3_621() {
    assert_eq!(least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
}
