// https://leetcode.com/problems/flower-planting-with-no-adjacent/
// Runtime: 24 ms
// Memory Usage: 3.2 MB
pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for path in paths {
        let u = (path[0] - 1) as usize;
        let v = (path[1] - 1) as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let mut colors: Vec<i32> = vec![0; n];
    for i in 0..n {
        let mut used: Vec<bool> = vec![false; 5];
        for &j in &g[i] {
            used[colors[j] as usize] = true;
        }
        for (c, &u) in used.iter().enumerate().take(5).skip(1) {
            if !u {
                colors[i] = c as i32;
                break;
            }
        }
    }
    colors
}
// graph
#[test]
fn test1_1042() {
    assert_eq!(
        garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]),
        vec![1, 2, 3]
    );
    assert_eq!(
        garden_no_adj(4, vec![vec![1, 2], vec![3, 4]]),
        vec![1, 2, 1, 2]
    );
    assert_eq!(
        garden_no_adj(
            4,
            vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 1],
                vec![1, 3],
                vec![2, 4]
            ]
        ),
        vec![1, 2, 3, 4]
    );
}
