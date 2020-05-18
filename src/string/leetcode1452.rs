// https://leetcode.com/problems/people-whose-list-of-favorite-companies-is-not-a-subset-of-another-list/
pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
    todo!()
}
// string sort
#[test]
fn test1_1452() {
    assert_eq!(
        people_indexes(vec![
            vec![
                String::from("leetcode"),
                String::from("google"),
                String::from("facebook")
            ],
            vec![String::from("google"), String::from("microsoft")],
            vec![String::from("google"), String::from("facebook")],
            vec![String::from("google")],
            vec![String::from("amazon")]
        ]),
        vec![0, 1, 4]
    );
    assert_eq!(
        people_indexes(vec![
            vec![
                String::from("leetcode"),
                String::from("google"),
                String::from("facebook")
            ],
            vec![String::from("leetcode"), String::from("amazon")],
            vec![String::from("facebook"), String::from("google")]
        ]),
        vec![0, 1]
    );
    assert_eq!(
        people_indexes(vec![
            vec![String::from("leetcode")],
            vec![String::from("google")],
            vec![String::from("facebook")],
            vec![String::from("amazon")]
        ]),
        vec![0, 1, 2, 3]
    );
}
