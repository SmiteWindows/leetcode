// https://leetcode-cn.com/problems/people-whose-list-of-favorite-companies-is-not-a-subset-of-another-list/
// Runtime: 72 ms
// Memory Usage: 7.2 MB
use std::collections::HashSet;
pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
    let people = favorite_companies
        .into_iter()
        .map(|v| v.into_iter().collect())
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
    use leetcode_prelude::vec2_string;
    assert_eq!(
        people_indexes(vec2_string![
            ["leetcode", "google", "facebook"],
            ["google", "microsoft"],
            ["google", "facebook"],
            ["google"],
            ["amazon"]
        ]),
        vec![0, 1, 4]
    );
    assert_eq!(
        people_indexes(vec2_string![
            ["leetcode", "google", "facebook"],
            ["leetcode", "amazon"],
            ["facebook", "google"]
        ]),
        vec![0, 1]
    );
    assert_eq!(
        people_indexes(vec2_string![
            ["leetcode"],
            ["google"],
            ["facebook"],
            ["amazon"]
        ]),
        vec![0, 1, 2, 3]
    );
}
