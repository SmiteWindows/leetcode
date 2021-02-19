// https://leetcode-cn.com/problems/next-permutation/
// Runtime: 0 ms
// Memory Usage: 1.9 MB
pub fn next_permutation(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut i = n - 1;
    while i > 0 && nums[i - 1] >= nums[i] {
        i -= 1;
    }
    let mut j = i;
    let mut k = n - 1;
    while j < k {
        nums.swap(j, k);
        j += 1;
        k -= 1;
    }
    if i > 0 {
        k = i;
        i -= 1;
        while nums[k] <= nums[i] {
            k += 1;
        }
        nums.swap(i, k)
    }
}
// array
#[test]
fn test1_31() {
    let mut nums1 = vec![1, 2, 3];
    next_permutation(&mut nums1);
    assert_eq!(nums1, vec![1, 3, 2]);
    let mut nums2 = vec![3, 2, 1];
    next_permutation(&mut nums2);
    assert_eq!(nums2, vec![1, 2, 3]);
    let mut nums3 = vec![1, 1, 5];
    next_permutation(&mut nums3);
    assert_eq!(nums3, vec![1, 5, 1]);
}
