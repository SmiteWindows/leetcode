// https://leetcode-cn.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut jumps = vec![0; n + 1];
    for (i, &range) in ranges.iter().enumerate().take(n + 1) {
        let d = range;
        let l = 0.max(i as i32 - d) as usize;
        let r = n.min(i + d as usize);
        jumps[l] = jumps[l].max(r - l);
    }
    let mut end = 0;
    let mut reach = 0;
    let mut res = 0;
    for (i, &jump) in jumps.iter().enumerate().take(n) {
        if i > reach {
            return -1;
        }
        reach = reach.max(i + jump);
        if i == end {
            res += 1;
            end = reach;
        }
    }
    if reach >= n as usize {
        res
    } else {
        -1
    }
}
// greedy dynamic_programming
#[test]
fn test2_1326() {
    assert_eq!(min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
    assert_eq!(min_taps(3, vec![0, 0, 0, 0]), -1);
    assert_eq!(min_taps(7, vec![1, 2, 1, 0, 2, 1, 0, 1]), 3);
    assert_eq!(min_taps(8, vec![4, 0, 0, 0, 0, 0, 0, 0, 4]), 2);
    assert_eq!(min_taps(8, vec![4, 0, 0, 0, 4, 0, 0, 0, 4]), 1);
}
