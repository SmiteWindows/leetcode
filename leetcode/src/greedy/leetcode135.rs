// https://leetcode.com/problems/candy/
// Runtime: 4 ms
// Memory Usage: 2.3 MB
pub fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut count = vec![1; n];
    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            count[i] = count[i].max(count[i - 1] + 1);
        }
    }
    for i in (0..n - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            count[i] = count[i].max(count[i + 1] + 1);
        }
    }
    count.into_iter().sum()
}
// greedy
#[test]
fn test1_135() {
    assert_eq!(candy(vec![1, 0, 2]), 5);
    assert_eq!(candy(vec![1, 2, 2]), 4);
}
