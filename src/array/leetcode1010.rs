// https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/
// Runtime: 4 ms
// Memory Usage: 2.5 MB
pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
    let mut a = vec![0; 60];
    let mut res = 0;
    for x in time {
        let count = a[((600 - x) % 60) as usize];
        if count != 0 {
            res += count;
        }
        a[(x % 60) as usize] += 1;
    }
    res
}
// array
#[test]
fn test1_1010() {
    assert_eq!(num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]), 3);
    assert_eq!(num_pairs_divisible_by60(vec![60, 60, 60]), 3);
}
