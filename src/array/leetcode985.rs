// https://leetcode.com/problems/sum-of-even-numbers-after-queries/
// Runtime: 20 ms
// Memory Usage: 3 MB
pub fn sum_even_after_queries(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut a = a;
    let mut sum = a.iter().filter(|&x| x % 2 == 0).sum();
    let mut res = vec![];
    for query in queries {
        let val = query[0];
        let index = query[1] as usize;
        let x = a[index];
        let y = a[index] + val;
        if x % 2 == 0 {
            sum -= x;
        }
        if y % 2 == 0 {
            sum += y;
        }
        a[index] = y;
        res.push(sum);
    }
    res
}
// array
#[test]
fn test1_985() {
    assert_eq!(
        sum_even_after_queries(
            vec![1, 2, 3, 4],
            vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]]
        ),
        vec![8, 6, 2, 4]
    );
}
