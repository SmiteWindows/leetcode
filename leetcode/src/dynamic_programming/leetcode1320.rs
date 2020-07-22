// https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers/
// Runtime: 20 ms
// Memory Usage: 3.3 MB
pub fn minimum_distance(word: String) -> i32 {
    let n = word.len();
    let s = word.bytes().map(|b| (b - b'A') as i32).collect::<Vec<_>>();
    let mut memo = vec![vec![vec![-1; 27]; 27]; n];
    dp(0, 26, 26, &mut memo, &s, n)
}

fn dp(start: usize, f1: i32, f2: i32, memo: &mut Vec<Vec<Vec<i32>>>, s: &[i32], n: usize) -> i32 {
    if start == n {
        0
    } else {
        if memo[start][f1 as usize][f2 as usize] != -1 {
            return memo[start][f1 as usize][f2 as usize];
        }
        let mut res = i32::MAX;
        let g = s[start];
        res = res.min(dp(start + 1, g, f2, memo, s, n) + dist(f1, g));
        res = res.min(dp(start + 1, f1, g, memo, s, n) + dist(f2, g));
        memo[start][f1 as usize][f2 as usize] = res;
        res
    }
}

fn dist(f: i32, g: i32) -> i32 {
    if f == 26 {
        0
    } else {
        (f / 6 - g / 6).abs() + (f % 6 - g % 6).abs()
    }
}
// dynamic_programming
#[test]
fn test1_1320() {
    assert_eq!(minimum_distance(String::from("CAKE")), 3);
    assert_eq!(minimum_distance(String::from("HAPPY")), 6);
    assert_eq!(minimum_distance(String::from("NEW")), 3);
    assert_eq!(minimum_distance(String::from("YEAR")), 7);
}
