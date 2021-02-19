// https://leetcode-cn.com/problems/shortest-distance-to-a-character/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let s = s.chars().collect::<Vec<_>>();
    let mut prev = None;
    let n = s.len();
    let mut res = vec![n as i32; n];
    for i in 0..n {
        if s[i] == c {
            prev = Some(i);
        }
        if let Some(j) = prev {
            res[i] = res[i].min((i - j) as i32);
        }
    }
    prev = None;
    for i in (0..n).rev() {
        if s[i] == c {
            prev = Some(i);
        }
        if let Some(j) = prev {
            res[i] = res[i].min((j - i) as i32);
        }
    }
    res
}
#[test]
fn test821() {
    assert_eq!(
        shortest_to_char("loveleetcode".to_string(), 'e'),
        vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
    );
}
