// https://leetcode-cn.com/problems/longest-continuous-increasing-subsequence/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut i: usize = 0;
    let mut j: usize = i;
    let mut res: usize = 1;
    while i < n {
        while j + 1 < n && nums[j + 1] > nums[j] {
            j += 1;
        }
        res = res.max(j - i + 1);
        i = j + 1;
        j = i;
    }
    res as i32
}
// array
#[test]
fn test1_674() {
    assert_eq!(find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
    assert_eq!(find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
}
