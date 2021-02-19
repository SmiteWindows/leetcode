// https://leetcode-cn.com/problems/relative-ranks/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
use std::cmp::Reverse;
pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
    let mut av = Vec::new();
    for (i, &num) in nums.iter().enumerate() {
        av.push(Athlete {
            index: i,
            score: num,
            rank: "".to_string(),
        });
    }
    av.sort_unstable_by_key(|a| Reverse(a.score));
    for (i, a) in av.iter_mut().enumerate() {
        a.rank = match i {
            0 => "Gold Medal".to_string(),
            1 => "Silver Medal".to_string(),
            2 => "Bronze Medal".to_string(),
            _ => format!("{}", i + 1),
        }
    }
    av.sort_unstable_by_key(|a| a.index);
    av.into_iter().map(|a| a.rank).collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Athlete {
    index: usize,
    score: i32,
    rank: String,
}

#[test]
fn test506() {
    assert_eq!(
        find_relative_ranks(vec![5, 4, 3, 2, 1]),
        vec![
            "Gold Medal".to_string(),
            "Silver Medal".to_string(),
            "Bronze Medal".to_string(),
            "4".to_string(),
            "5".to_string()
        ]
    );
}
