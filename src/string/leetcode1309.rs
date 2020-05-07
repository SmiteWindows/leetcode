// https://leetcode.com/problems/decrypt-string-from-alphabet-to-integer-mapping/
pub fn freq_alphabets(s: String) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1309() {
    assert_eq!(
        freq_alphabets(String::from("10#11#12")),
        String::from("jkab")
    );
    assert_eq!(freq_alphabets(String::from("1326#")), String::from("acz"));
    assert_eq!(freq_alphabets(String::from("25#")), String::from("y"));
    assert_eq!(
        freq_alphabets(String::from(
            "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#"
        )),
        String::from("abcdefghijklmnopqrstuvwxyz")
    );
}
