// https://leetcode.com/problems/can-convert-string-in-k-moves/
pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
    todo!()
}
// string greedy
#[test]
#[ignore]
fn test1_1540() {
    assert_eq!(
        can_convert_string("input".to_string(), "ouput".to_string(), 9),
        true
    );
    assert_eq!(
        can_convert_string("abc".to_string(), "bcd".to_string(), 10),
        false
    );
    assert_eq!(
        can_convert_string("aab".to_string(), "bbb".to_string(), 27),
        true
    );
}
