// https://leetcode.com/problems/string-matching-in-an-array/
pub fn string_matching(words: Vec<String>) -> Vec<String> {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1408() {
    assert_eq!(
        string_matching(vec![
            String::from("mass"),
            String::from("as"),
            String::from("hero"),
            String::from("superhero")
        ]),
        vec![String::from("as"), String::from("hero")]
    );
    assert_eq!(
        string_matching(vec![
            String::from("leetcode"),
            String::from("et"),
            String::from("code")
        ]),
        vec![String::from("et"), String::from("code")]
    );
    assert_eq!(
        string_matching(vec![
            String::from("blue"),
            String::from("green"),
            String::from("bu")
        ]),
        vec![String::new()]
    );
}
