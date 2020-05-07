// https://leetcode.com/problems/find-lucky-integer-in-an-array/
pub fn find_lucky(arr: Vec<i32>) -> i32 {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1394() {
    assert_eq!(find_lucky(vec![2, 2, 3, 4]), 2);
    assert_eq!(find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    assert_eq!(find_lucky(vec![2, 2, 2, 3, 3]), -1);
    assert_eq!(find_lucky(vec![5]), -1);
    assert_eq!(find_lucky(vec![7, 7, 7, 7, 7, 7, 7]), 7);
}
