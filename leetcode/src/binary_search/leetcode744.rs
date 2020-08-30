// https://leetcode-cn.com/problems/find-smallest-letter-greater-than-target/
// Runtime: 4 ms
// Memory Usage: 2.7 MB
pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    letters.binary_search(&target).map_or_else(
        |i| letters[i % letters.len()],
        |i| letters[(i + 1) % letters.len()],
    )
}
// binary_search
#[test]
fn test1_744() {
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'd'), 'f');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'g'), 'j');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'j'), 'c');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'k'), 'c');
}
