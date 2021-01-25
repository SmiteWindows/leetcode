// https://leetcode-cn.com/problems/path-crossing/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::collections::HashSet;
pub fn is_path_crossing(path: String) -> bool {
    let mut hs = HashSet::new();
    hs.insert((0, 0));
    let mut x = 0;
    let mut y = 0;
    for c in path.chars() {
        match c {
            'N' => {
                y += 1;
            }
            'S' => {
                y -= 1;
            }
            'E' => {
                x += 1;
            }
            _ => {
                x -= 1;
            }
        }
        if !hs.insert((x, y)) {
            return true;
        }
    }
    false
}
// string
#[test]
fn test1_1496() {
    assert_eq!(is_path_crossing("NES".to_string()), false);
    assert_eq!(is_path_crossing("NESWW".to_string()), true);
}
