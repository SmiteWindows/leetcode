// https://leetcode.com/problems/assign-cookies/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut g = g;
    let mut s = s;
    g.sort_unstable();
    s.sort_unstable();
    let mut i = 0;
    let mut j = 0;
    while i < g.len() && j < s.len() {
        if g[i] <= s[j] {
            i += 1;
            j += 1;
        } else {
            while j < s.len() && s[j] < g[i] {
                j += 1;
            }
        }
    }
    i as i32
}
// greedy
#[test]
fn test1_455() {
    assert_eq!(find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
    assert_eq!(find_content_children(vec![1, 2], vec![1, 2, 3]), 2);
}
