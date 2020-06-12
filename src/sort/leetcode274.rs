// https://leetcode.com/problems/h-index/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn h_index(citations: Vec<i32>) -> i32 {
    let n = citations.len();
    let mut count = vec![0; n + 1];
    for c in citations {
        let i = c as usize;
        if i < n {
            count[i] += 1;
        } else {
            count[n] += 1;
        }
    }
    let mut h = n;
    let mut sum = 0;
    while sum <= h {
        sum += count[h];
        if sum >= h {
            return h as i32;
        }
        h -= 1;
    }
    0
}
// sort hash_table
#[test]
fn test1_274() {
    assert_eq!(h_index(vec![3, 0, 6, 1, 5]), 3);
}
