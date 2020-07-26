// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/
pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    todo!()
}
// math array
#[test]
#[ignore]
fn test1_1524() {
    assert_eq!(num_of_subarrays(vec![1, 3, 5]), 4);
    assert_eq!(num_of_subarrays(vec![2, 4, 6]), 0);
    assert_eq!(num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
    assert_eq!(num_of_subarrays(vec![100, 100, 99, 99]), 4);
    assert_eq!(num_of_subarrays(vec![7]), 1);
}
