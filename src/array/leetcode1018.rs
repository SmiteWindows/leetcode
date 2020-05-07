// https://leetcode.com/problems/binary-prefix-divisible-by-5/
pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1018() {
    assert_eq!(prefixes_div_by5(vec![0, 1, 1]), vec![true, false, false]);
    assert_eq!(prefixes_div_by5(vec![1, 1, 1]), vec![false, false, false]);
    assert_eq!(
        prefixes_div_by5(vec![0, 1, 1, 1, 1, 1]),
        vec![true, false, false, false, true, false]
    );
    assert_eq!(
        prefixes_div_by5(vec![1, 1, 1, 0, 1]),
        vec![false, false, false, false, false]
    );
}
