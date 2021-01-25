// https://leetcode-cn.com/problems/check-if-all-1s-are-at-least-length-k-places-away/
// Runtime: 16 ms
// Memory Usage: 2.6 MB
pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let mut prev = None;
    let k = k as usize;
    for (i, &num) in nums.iter().enumerate() {
        if num == 1 {
            if let Some(j) = prev {
                if i - j <= k {
                    return false;
                }
            }
            prev = Some(i)
        }
    }
    true
}
// array
#[test]
fn test1_1437() {
    assert_eq!(k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2), true);
    assert_eq!(k_length_apart(vec![1, 0, 0, 1, 0, 1], 2), false);
    assert_eq!(k_length_apart(vec![1, 1, 1, 1, 1], 0), true);
    assert_eq!(k_length_apart(vec![0, 1, 0, 1], 1), true);
}
