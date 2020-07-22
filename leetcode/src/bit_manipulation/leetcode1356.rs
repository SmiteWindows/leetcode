// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    arr.sort_unstable_by_key(|&x| (x.count_ones(), x));
    arr
}
// sort bit_manipulation
#[test]
fn test2_1356() {
    assert_eq!(
        sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
    );
    assert_eq!(
        sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
        vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
    );
    assert_eq!(sort_by_bits(vec![10000, 10000]), vec![10000, 10000]);
    assert_eq!(
        sort_by_bits(vec![2, 3, 5, 7, 11, 13, 17, 19]),
        vec![2, 3, 5, 17, 7, 11, 13, 19]
    );
    assert_eq!(
        sort_by_bits(vec![10, 100, 1000, 10000]),
        vec![10, 100, 10000, 1000]
    );
}
