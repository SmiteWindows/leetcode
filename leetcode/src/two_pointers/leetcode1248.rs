// https://leetcode-cn.com/problems/count-number-of-nice-subarrays/
// Runtime: 12 ms
// Memory Usage: 2.5 MB
pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut count = vec![0; n + 1];
    let k = k as usize;
    count[0] = 1;
    let mut prev = 0;
    let mut res = 0;
    for num in nums.iter().take(n) {
        if num % 2 == 1 {
            prev += 1;
        }
        count[prev] += 1;
        if prev >= k {
            res += count[prev - k];
        }
    }
    res
}
// two_pointers
#[test]
fn test1_1248() {
    assert_eq!(number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
    assert_eq!(number_of_subarrays(vec![2, 4, 6], 1), 0);
    assert_eq!(
        number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
        16
    );
}
