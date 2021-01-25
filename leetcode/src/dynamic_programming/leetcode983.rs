// https://leetcode-cn.com/problems/minimum-cost-for-tickets/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let n = days.len();
    let mut dp: Vec<i32> = vec![];
    let pass = vec![1, 7, 30];
    for i in 0..n {
        let mut mins = costs.clone();
        for k in 0..3 {
            for j in (0..i).rev() {
                if days[i] - days[j] >= pass[k] {
                    mins[k] += dp[j];
                    break;
                }
            }
        }
        dp.push(*mins.iter().min().unwrap());
    }
    dp[n - 1]
}
// dynamic_programming
#[test]
fn test1_983() {
    assert_eq!(mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]), 11);
    assert_eq!(
        mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
        17
    );
}
