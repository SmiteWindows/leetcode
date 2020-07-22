// https://leetcode.com/problems/path-in-zigzag-labelled-binary-tree/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
    let mut label = label;
    let mut level = 0;
    while label >= 1 << level {
        level += 1;
    }
    let mut res = vec![0; level];
    for i in (0..level).rev() {
        res[i] = label;
        let r = (1 << i) - 1;
        let l = r / 2 + 1;
        label = l + r - label / 2;
    }
    res
}
// tree math
#[test]
fn test2_1104() {
    assert_eq!(path_in_zig_zag_tree(14), vec![1, 3, 4, 14]);
    assert_eq!(path_in_zig_zag_tree(26), vec![1, 2, 6, 10, 26]);
}
