// https://leetcode-cn.com/problems/find-the-duplicate-number/
// Runtime: 0 ms
// Memory Usage: 2.2 MB
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let n = (nums.len() - 1) as i32;
    let mut low = 1;
    let mut high = n;
    while low < high {
        let mid = low + (high - low) / 2;
        let mut count = 0;
        for &x in &nums {
            if x <= mid {
                count += 1;
            }
        }
        if count <= mid {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}
// array two_pointers binary_search
#[test]
fn test1_287() {
    assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
}
