// https://leetcode-cn.com/problems/majority-element/
// Runtime: 0 ms
// Memory Usage: 2.3 MB
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut majority = None;
    let mut count = 0;
    for num in nums {
        if count == 0 {
            majority = Some(num);
        }

        count = if majority == Some(num) {
            count + 1
        } else {
            count - 1
        };
    }
    majority.unwrap_or_default()
}
// bit_manipulation array divide_and_conquer
#[test]
fn test2_169() {
    assert_eq!(majority_element(vec![3, 2, 3]), 3);
    assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
