// https://leetcode-cn.com/problems/pyramid-transition-matrix/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::collections::HashSet;
pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
    let n = bottom.len();
    let mut v = vec![vec![]; n];
    let mut map = vec![vec![HashSet::new(); 7]; 7];
    for c in bottom.bytes() {
        let b = (c - b'A') as usize;
        v[n - 1].push(b);
    }
    for s in allowed {
        let s = s.bytes().collect::<Vec<u8>>();
        let a = (s[0] - b'A') as usize;
        let b = (s[1] - b'A') as usize;
        let c = (s[2] - b'A') as usize;
        map[a][b].insert(c);
    }
    backtrack(&mut v, &map, n - 1, n - 1)
}

fn backtrack(v: &mut Vec<Vec<usize>>, map: &[Vec<HashSet<usize>>], row: usize, col: usize) -> bool {
    if row == 0 {
        return true;
    }
    let (r, c) = if col == row {
        (row - 1, 0)
    } else {
        (row, col + 1)
    };
    let left = v[r + 1][c];
    let right = v[r + 1][c + 1];
    for &x in &map[left][right] {
        v[r].push(x);
        if backtrack(v, map, r, c) {
            return true;
        }
        v[r].pop();
    }
    false
}
// bit_manipulation depth_first_search
#[test]
fn test1_756() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        pyramid_transition("BCD".to_string(), vec_string!["BCG", "CDE", "GEA", "FFF"]),
        true
    );
    assert_eq!(
        pyramid_transition(
            "AABA".to_string(),
            vec_string!["AAA", "AAB", "ABA", "ABB", "BAC"]
        ),
        false
    );
}
