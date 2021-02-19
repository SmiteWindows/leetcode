// https://leetcode-cn.com/problems/reverse-string/
// Runtime: 20 ms
// Memory Usage: 5.3 MB
pub fn reverse_string(s: &mut Vec<char>) {
    let n = s.len();
    if n == 0 {
        return;
    }
    let mut i = 0;
    let mut j = n - 1;
    while i < j {
        s.swap(i, j);
        i += 1;
        j -= 1;
    }
}
// string two_pointers
#[test]
fn test1_344() {
    let mut s1 = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s1);
    assert_eq!(s1, vec!['o', 'l', 'l', 'e', 'h']);
    let mut s1 = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    reverse_string(&mut s1);
    assert_eq!(s1, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}
