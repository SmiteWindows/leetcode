// https://leetcode.com/problems/lexicographical-numbers/
// Runtime: 8 ms
// Memory Usage: 2.6 MB
pub fn lexical_order(n: i32) -> Vec<i32> {
    let mut res = vec![];
    for i in 1..10 {
        dfs(i, &mut res, n);
    }
    res
}

fn dfs(cur: i32, all: &mut Vec<i32>, n: i32) {
    if cur > n {
        return;
    }
    all.push(cur);
    for i in 0..10 {
        dfs(cur * 10 + i, all, n);
    }
}
#[test]
fn test386() {
    assert_eq!(
        lexical_order(13),
        vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
    );
}
