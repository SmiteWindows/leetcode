// https://leetcode-cn.com/problems/minimum-swaps-to-make-strings-equal/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn minimum_swap(s1: String, s2: String) -> i32 {
    let n = s1.len();
    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();
    let mut x = 0;
    let mut y = 0;
    for i in 0..n {
        if s1[i] == 'x' && s2[i] == 'y' {
            x += 1;
        }
        if s1[i] == 'y' && s2[i] == 'x' {
            y += 1;
        }
    }
    if (x + y) % 2 != 0 {
        return -1;
    }
    x / 2 + y / 2 + x % 2 * 2
}
// greedy string
#[test]
fn test2_1247() {
    assert_eq!(minimum_swap("xx"), "yy")), 1);
    assert_eq!(minimum_swap("xy"), "yx")), 2);
    assert_eq!(minimum_swap("xx"), "xy")), -1);
    assert_eq!(
        minimum_swap("xxyyxyxyxx"), "xyyxyxxxyx")),
        4
    );
}
