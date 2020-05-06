// https://leetcode.com/problems/gas-station/
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    todo!()
}
// greedy
#[test]
#[ignore]
fn test1_134() {
    assert_eq!(
        can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
        3
    );
    assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
}
