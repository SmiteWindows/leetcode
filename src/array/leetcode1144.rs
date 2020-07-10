// https://leetcode.com/problems/decrease-elements-to-make-array-zigzag/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
    let mut sums = vec![0, 0];
    let n = nums.len();
    for i in 0..n {
        let mut adj = vec![];
        if i > 0 {
            adj.push(nums[i - 1]);
        }
        if i + 1 < n {
            adj.push(nums[i + 1]);
        }
        let min = adj.into_iter().min().unwrap();
        if nums[i] >= min {
            sums[i % 2] += (nums[i] - min) + 1;
        }
    }
    sums[0].min(sums[1])
}
// array
#[test]
fn test1_1144() {
    assert_eq!(moves_to_make_zigzag(vec![1, 2, 3]), 2);
    assert_eq!(moves_to_make_zigzag(vec![9, 6, 1, 6, 2]), 4);
}
