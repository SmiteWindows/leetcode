// https://leetcode.com/problems/diagonal-traverse-ii/
pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
       todo!() 
}
// array sort
#[test]
#[ignore]
fn test1_1424() {
    assert_eq!(find_diagonal_order(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]),vec![1,4,2,7,5,3,8,6,9]);
    assert_eq!(find_diagonal_order(vec![vec![1,2,3,4,5],vec![6,7],vec![8]],vec![9,10,11],vec![12,13,14,15,16]),vec![1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]);
    assert_eq!(find_diagonal_order(vec![vec![1,2,3],vec![4],vec![5,6,7],vec![8],vec![9,10,11]]),vec![1,4,2,5,3,8,6,9,7,10,11]);
    assert_eq!(find_diagonal_order(vec![vec![1,2,3,4,5,6]]),vec![1,2,3,4,5,6]);
}