// https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/
pub fn count_triplets(arr: Vec<i32>) -> i32 {
    todo!()
}
// array math bit_manipulation
#[test]
#[ignore]
fn test2_1442() {
    assert_eq!(count_triplets(vec![2, 3, 1, 6, 7]), 4);
    assert_eq!(count_triplets(vec![1, 1, 1, 1, 1]), 10);
    assert_eq!(count_triplets(vec![2, 3]), 0);
    assert_eq!(count_triplets(vec![1, 3, 5, 7, 9]), 3);
    assert_eq!(count_triplets(vec![7, 11, 12, 9, 5, 2, 7, 17, 22]), 8);
}