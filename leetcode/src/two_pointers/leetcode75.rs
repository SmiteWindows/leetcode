// https://leetcode-cn.com/problems/sort-colors/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn sort_colors(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut l = 0;
    let mut r = n - 1;
    let mut i = 0;
    while i <= r {
        while nums[i] == 2 && i < r {
            nums.swap(i, r);
            r -= 1;
        }
        while nums[i] == 0 && i > l {
            nums.swap(i, l);
            l += 1;
        }
        i += 1;
    }
}
// array two_pointers sort
#[test]
fn test1_75() {
    let mut nums1 = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums1);
    assert_eq!(nums1, vec![0, 0, 1, 1, 2, 2]);
}
