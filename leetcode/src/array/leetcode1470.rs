// https://leetcode-cn.com/problems/shuffle-the-array/
pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    nums.iter()
        .skip(n as usize)
        .zip(nums.iter())
        .map(|(&x, &y)| vec![y, x])
        .flatten()
        .collect::<Vec<i32>>()
}
// Runtime: 0 ms
// Memory Usage: 2.1 MB
// ✔
// array
#[test]
fn test1_1470() {
    assert_eq!(shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
    assert_eq!(
        shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
        vec![1, 4, 2, 3, 3, 2, 4, 1]
    );
    assert_eq!(shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
}
