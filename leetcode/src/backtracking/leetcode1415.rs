// https://leetcode-cn.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn get_happy_string(n: i32, k: i32) -> String {
    let n = n as usize;
    let k = k as usize;
    let mut all = Vec::new();
    let mut cur = Vec::new();
    dfs(0, &mut cur, &mut all, n, k);
    if all.len() < k {
        "".to_string()
    } else {
        all.pop().unwrap()
    }
}

fn dfs(start: usize, cur: &mut Vec<u8>, all: &mut Vec<String>, n: usize, k: usize) {
    if all.len() == k {
        return;
    }
    if start == n {
        let s = cur.iter().map(|&i| (b'a' + i) as char).collect();
        all.push(s);
    } else {
        for i in 0..3 {
            if start > 0 && i == cur[start - 1] {
                continue;
            }
            cur.push(i);
            dfs(start + 1, cur, all, n, k);
            cur.pop();
        }
    }
}
// backtracking
#[test]
fn test1_1415() {
    assert_eq!(get_happy_string(1, 3), "c".to_string());
    assert_eq!(get_happy_string(1, 4), "".to_string());
    assert_eq!(get_happy_string(3, 9), "cab".to_string());
    assert_eq!(get_happy_string(2, 7), "".to_string());
    assert_eq!(get_happy_string(10, 100), "abacbabacb".to_string());
}
