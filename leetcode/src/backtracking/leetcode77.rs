// https://leetcode-cn.com/problems/combinations/
// Runtime: 4 ms
// Memory Usage: 2.9 MB
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut cur = Vec::new();
    let k = k as usize;
    let n = n as usize;
    dfs(1, &mut cur, &mut res, k, n);
    res
}

fn dfs(start: usize, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, k: usize, n: usize) {
    if cur.len() == k {
        res.push(cur.to_vec());
    } else {
        for i in start..=n - (k - cur.len()) + 1 {
            cur.push(i as i32);
            dfs(i + 1, cur, res, k, n);
            cur.pop();
        }
    }
}
// backtracking
#[test]
fn test1_77() {
    use leetcode_prelude::vec2;
    assert_eq!(
        combine(4, 2),
        vec2![[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
    );
}
