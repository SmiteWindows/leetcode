// https://leetcode-cn.com/problems/reverse-string/
// Runtime: 20 ms
// Memory Usage: 5.3 MB
pub fn reverse_string(s: &mut Vec<char>) {
    s.reverse();
}
// string two_pointers
#[test]
fn test2_344() {
    let mut s1 = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s1);
    assert_eq!(s1, vec!['o', 'l', 'l', 'e', 'h']);
    let mut s1 = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    reverse_string(&mut s1);
    assert_eq!(s1, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}
