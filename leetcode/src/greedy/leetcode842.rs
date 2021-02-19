// https://leetcode-cn.com/problems/split-array-into-fibonacci-sequence/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn split_into_fibonacci(s: String) -> Vec<i32> {
    let mut cur = Vec::new();
    let mut res = Vec::new();
    dfs(0, &mut cur, &mut res, &s, s.len());
    res
}

fn dfs(start: usize, cur: &mut Vec<i32>, res: &mut Vec<i32>, s: &str, end: usize) {
    if start == end && cur.len() > 2 {
        *res = cur.to_vec();
    } else {
        for i in 1..=(end - start) {
            let t = &s[start..start + i];
            if &s[start..start + 1] == "0" && i > 1 {
                break;
            }
            if let Ok(x) = t.parse::<i32>() {
                let n = cur.len();
                if n < 2 {
                    cur.push(x);
                    dfs(start + i, cur, res, s, end);
                    cur.pop();
                } else {
                    if x > cur[n - 1] + cur[n - 2] {
                        break;
                    }
                    if x == cur[n - 1] + cur[n - 2] {
                        cur.push(x);
                        dfs(start + i, cur, res, s, end);
                        cur.pop();
                    }
                }
            } else {
                break;
            }
        }
    }
}
// backtracking string greedy
#[test]
fn test2_842() {
    assert_eq!(
        split_into_fibonacci("123456579".to_string()),
        vec![123, 456, 579]
    );
    assert_eq!(
        split_into_fibonacci("11235813".to_string()),
        vec![1, 1, 2, 3, 5, 8, 13]
    );
    assert_eq!(split_into_fibonacci("112358130".to_string()), vec![]);
    assert_eq!(split_into_fibonacci("0123".to_string()), vec![]);
    assert_eq!(
        split_into_fibonacci("1101111".to_string()),
        vec![110, 1, 111]
    );
}
