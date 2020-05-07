// https://leetcode.com/problems/string-compression/
pub fn compress(chars: &mut Vec<char>) -> i32 {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_443() {
    let mut chars1 = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    assert_eq!(compress(&mut chars1), 6);
    let mut chars2 = vec!['a'];
    assert_eq!(compress(&mut chars2), 1);
    let mut chars3 = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    ];
    assert_eq!(compress(&mut chars3), 4);
}
