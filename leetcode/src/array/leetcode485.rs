// https://leetcode.com/problems/max-consecutive-ones/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut count = 0;
    for x in nums {
        if x == 1 {
            count += 1;
            max = max.max(count);
        } else {
            count = 0;
        }
    }
    max
}
// array
#[test]
fn test1_485() {
    assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
}
