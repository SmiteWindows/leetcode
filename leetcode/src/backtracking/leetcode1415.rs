// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn get_happy_string(n: i32, k: i32) -> String {
    let n = n as usize;
    let k = k as usize;
    let mut all = vec![];
    let mut cur = vec![];
    dfs(0, &mut cur, &mut all, n, k);
    if all.len() < k {
        "".to_string()
    } else {
        all.pop().expect("exist")
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
    assert_eq!(get_happy_string(1, 3), String::from("c"));
    assert_eq!(get_happy_string(1, 4), String::from(""));
    assert_eq!(get_happy_string(3, 9), String::from("cab"));
    assert_eq!(get_happy_string(2, 7), String::from(""));
    assert_eq!(get_happy_string(10, 100), String::from("abacbabacb"));
}
