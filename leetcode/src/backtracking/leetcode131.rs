// https://leetcode-cn.com/problems/palindrome-partitioning/
// Runtime: 4 ms
// Memory Usage: 2.9 MB
pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut res = vec![];
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut indexes = vec![];
    dfs(0, &s, &mut indexes, &mut res, n);
    res
}

fn dfs(
    start: usize,
    s: &[char],
    indexes: &mut Vec<(usize, usize)>,
    res: &mut Vec<Vec<String>>,
    n: usize,
) {
    if start == n {
        let mut partition = vec![];
        for &(l, r) in indexes.iter() {
            partition.push(s[l..r].iter().collect());
        }
        res.push(partition);
    }
    for end in start + 1..=n {
        if is_palindrome(&s[start..end]) {
            indexes.push((start, end));
            dfs(end, s, indexes, res, n);
            indexes.pop();
        }
    }
}

fn is_palindrome(s: &[char]) -> bool {
    let n = s.len();
    for i in 0..n {
        let j = n - i - 1;
        if s[i] != s[j] {
            return false;
        }
    }
    true
}
// backtracking
#[test]
fn test1_131() {
    use leetcode_prelude::vec2_string;
    assert_eq!(
        partition("aab".to_string()),
        vec2_string![["a", "a", "b"], ["aa", "b"]]
    );
}
