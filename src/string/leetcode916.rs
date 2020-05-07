// https://leetcode.com/problems/word-subsets/
pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_916() {
    assert_eq!(
        word_subsets(
            vec![
                String::from("amazon"),
                String::from("apple"),
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ],
            vec![String::from("e"), String::from("o")]
        ),
        vec![
            String::from("facebook"),
            String::from("google"),
            String::from("leetcode")
        ]
    );
    assert_eq!(
        word_subsets(
            vec![
                String::from("amazon"),
                String::from("apple"),
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ],
            vec![String::from("l"), String::from("e")]
        ),
        vec![
            String::from("apple"),
            String::from("google"),
            String::from("leetcode")
        ]
    );
    assert_eq!(
        word_subsets(
            vec![
                String::from("amazon"),
                String::from("apple"),
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ],
            vec![String::from("e"), String::from("oo")]
        ),
        vec![String::from("facebook"), String::from("google")]
    );
    assert_eq!(
        word_subsets(
            vec![
                String::from("amazon"),
                String::from("apple"),
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ],
            vec![String::from("lo"), String::from("eo")]
        ),
        vec![String::from("google"), String::from("leetcode")]
    );
    assert_eq!(
        word_subsets(
            vec![
                String::from("amazon"),
                String::from("apple"),
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ],
            vec![String::from("ec"), String::from("oc"), String::from("ceo")]
        ),
        vec![String::from("facebook"), String::from("leetcode")]
    );
}
