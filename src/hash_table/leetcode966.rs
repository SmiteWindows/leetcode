// https://leetcode.com/problems/vowel-spellchecker/
pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    todo!()
}
// hash_table string
#[test]
#[ignore]
fn test1_966() {
    assert_eq!(
        spellchecker(
            vec![
                String::from("KiTe"),
                String::from("kite"),
                String::from("hare"),
                String::from("Hare")
            ],
            vec![
                String::from("kite"),
                String::from("Kite"),
                String::from("KiTe"),
                String::from("Hare"),
                String::from("HARE"),
                String::from("Hear"),
                String::from("hear"),
                String::from("keti"),
                String::from("keet"),
                String::from("keto")
            ]
        ),
        vec![
            String::from("kite"),
            String::from("KiTe"),
            String::from("KiTe"),
            String::from("Hare"),
            String::from("hare"),
            String::from(""),
            String::from(""),
            String::from("KiTe"),
            String::from(""),
            String::from("KiTe")
        ]
    );
}
