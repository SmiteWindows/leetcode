// https://leetcode-cn.com/problems/people-whose-list-of-favorite-companies-is-not-a-subset-of-another-list/
// Runtime: 72 ms
// Memory Usage: 7.2 MB
use std::{collections::HashSet, iter::FromIterator};
pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
    let people = favorite_companies
        .into_iter()
        .map(|v| HashSet::from_iter(v.into_iter()))
        .collect::<Vec<HashSet<_>>>();
    let n = people.len();
    let mut res = vec![];
    'outer: for i in 0..n {
        for j in 0..n {
            if i != j && people[i].is_subset(&people[j]) {
                continue 'outer;
            }
        }
        res.push(i as i32);
    }
    res
}
// string sort
#[test]
fn test2_1452() {
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
