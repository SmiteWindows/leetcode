// https://leetcode.com/problems/find-smallest-letter-greater-than-target/
pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    todo!()
}
// binary_search
#[test]
#[ignore]
fn test1_744() {
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'd'), 'f');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'g'), 'j');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'j'), 'c');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'k'), 'c');
}
