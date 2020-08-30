// https://leetcode-cn.com/problems/find-the-town-judge/
// Runtime: 20 ms
// Memory Usage: 2.7 MB
pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut degree = vec![0; n];
    for edge in trust {
        let u = (edge[0] - 1) as usize;
        let v = (edge[1] - 1) as usize;
        degree[v] += 1;
        degree[u] -= 1;
    }
    for (i, &d) in degree.iter().enumerate().take(n) {
        if d as usize == n - 1 {
            return (i + 1) as i32;
        }
    }
    -1
}
// graph
#[test]
fn test1_997() {
    assert_eq!(find_judge(2, vec![vec![1, 2]]), 2);
    assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]), -1);
    assert_eq!(find_judge(3, vec![vec![1, 2], vec![2, 3]]), -1);
    assert_eq!(
        find_judge(
            4,
            vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]]
        ),
        3
    );
}
