// https://leetcode-cn.com/problems/number-of-good-pairs/
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let (mut count, mut arr) = (0, vec![0; 101]);
    nums.iter().for_each(|&x| {
        count += arr[x as usize];
        arr[x as usize] += 1;
    });
    count
}
// Runtime: 0 ms
// Memory Usage: 2 MB
// ✔
// array hash_table math
#[test]
fn test1_1512() {
    assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    assert_eq!(num_identical_pairs(vec![1, 1, 1, 1]), 6);
    assert_eq!(num_identical_pairs(vec![1, 2, 3]), 0);
}
