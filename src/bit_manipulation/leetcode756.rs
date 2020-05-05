// https://leetcode.com/problems/pyramid-transition-matrix/
pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
    todo!()
}
// bit_manipulation depth_first_search
#[test]
#[ignore]
fn test1_756() {
    assert_eq!(
        pyramid_transition(
            String::from("BCD"),
            vec![
                String::from("BCG"),
                String::from("CDE"),
                String::from("GEA"),
                String::from("FFF")
            ]
        ),
        true
    );
    assert_eq!(
        pyramid_transition(
            String::from("AABA"),
            vec![
                String::from("AAA"),
                String::from("AAB"),
                String::from("ABA"),
                String::from("ABB"),
                String::from("BAC")
            ]
        ),
        false
    );
}
