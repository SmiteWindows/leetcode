// https://leetcode-cn.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn count_triplets(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut res = 0;
    for i in 0..n {
        let mut xor = arr[i];
        for (j, aj) in arr.iter().enumerate().take(n).skip(i + 1) {
            xor ^= aj;
            if xor == 0 {
                res += j - i;
            }
        }
    }
    res as i32
}
// array math bit_manipulation
#[test]
fn test3_1442() {
    assert_eq!(count_triplets(vec![2, 3, 1, 6, 7]), 4);
    assert_eq!(count_triplets(vec![1, 1, 1, 1, 1]), 10);
    assert_eq!(count_triplets(vec![2, 3]), 0);
    assert_eq!(count_triplets(vec![1, 3, 5, 7, 9]), 3);
    assert_eq!(count_triplets(vec![7, 11, 12, 9, 5, 2, 7, 17, 22]), 8);
}
