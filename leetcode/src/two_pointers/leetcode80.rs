// https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array-ii/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    let mut m = 0;
    let mut prev = None;
    for i in 0..n {
        if let Some((value, count)) = prev {
            if value == nums[i] {
                if count == 1 {
                    prev = Some((nums[i], 2));
                    nums[m] = nums[i];
                    m += 1;
                }
            } else {
                prev = Some((nums[i], 1));
                nums[m] = nums[i];
                m += 1;
            }
        } else {
            prev = Some((nums[i], 1));
            nums[m] = nums[i];
            m += 1;
        }
    }
    m as i32
}
// array two_pointers
#[test]
fn test1_80() {
    let mut nums1 = vec![1, 1, 1, 2, 2, 3];
    assert_eq!(remove_duplicates(&mut nums1), 5);
    let mut nums2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    assert_eq!(remove_duplicates(&mut nums2), 7);
}
