// https://leetcode.com/problems/minimum-cost-for-tickets/
pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_983() {
    assert_eq!(mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]), 11);
    assert_eq!(
        mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
        17
    );
}
