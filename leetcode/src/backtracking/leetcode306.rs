// https://leetcode-cn.com/problems/additive-number/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn is_additive_number(num: String) -> bool {
    let n = num.len();
    let mut res = false;
    let mut cur = vec![];
    dfs(0, &mut cur, &mut res, &num[..], n);
    res
}

fn dfs(start: usize, cur: &mut Vec<u64>, res: &mut bool, s: &str, n: usize) {
    if start == n {
        if cur.len() >= 3 {
            *res = true;
        }
    } else {
        for i in start + 1..=n {
            if &s[start..=start] == "0" && i - start > 1 {
                return;
            }
            if let Ok(x) = s[start..i].parse::<u64>() {
                let k = cur.len();
                if k < 2 || cur[k - 1] + cur[k - 2] == x {
                    cur.push(x);
                    dfs(i, cur, res, s, n);
                    cur.pop();
                }
            }
        }
    }
}
// backtracking
#[test]
fn test1_306() {
    assert_eq!(is_additive_number("112358".to_string()), true);
    assert_eq!(is_additive_number("199100199".to_string()), true);
}
