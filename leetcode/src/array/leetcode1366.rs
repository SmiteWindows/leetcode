// https://leetcode-cn.com/problems/rank-teams-by-votes/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn rank_teams(votes: Vec<String>) -> String {
    let n = votes[0].len();
    let mut count = vec![vec![0; n]; 26];
    for s in &votes {
        for (i, c) in s.bytes().enumerate() {
            count[(c - b'A') as usize][i] += 1;
        }
    }
    let mut v = votes[0].bytes().map(|b| b - b'A').collect::<Vec<_>>();
    v.sort_by(|&a, &b| {
        for i in 0..n {
            match &count[a as usize][i].cmp(&count[b as usize][i]) {
                Equal => {}
                Less => {
                    return Greater;
                }
                Greater => {
                    return Less;
                }
            }
        }
        a.cmp(&b)
    });
    v.into_iter().map(|b| (b'A' + b) as char).collect()
}
// sort array
#[test]
fn test2_1366() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        rank_teams(vec_string!["ABC", "ACB", "ABC", "ACB", "ACB"]),
        "ACB".to_string()
    );
    assert_eq!(rank_teams(vec_string!["WXYZ", "XYZW"]), "XWYZ".to_string());
    assert_eq!(
        rank_teams(vec_string!["ZMNAGUEDSJYLBOPHRQICWFXTVK"]),
        "ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string()
    );
    assert_eq!(
        rank_teams(vec_string!["BCA", "CAB", "CBA", "ABC", "ACB", "BAC"]),
        "ABC".to_string()
    );
    assert_eq!(rank_teams(vec_string!["M", "M", "M", "M"]), "M".to_string());
}
