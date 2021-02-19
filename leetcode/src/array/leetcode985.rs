// https://leetcode-cn.com/problems/sum-of-even-numbers-after-queries/
// Runtime: 20 ms
// Memory Usage: 3 MB
pub fn sum_even_after_queries(mut a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sum = a.iter().filter(|&x| x % 2 == 0).sum();
    let mut res = Vec::new();
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
    use leetcode_prelude::vec2;
    assert_eq!(
        sum_even_after_queries(vec![1, 2, 3, 4], vec2![[1, 0], [-3, 1], [-4, 0], [2, 3]]),
        vec![8, 6, 2, 4]
    );
}
