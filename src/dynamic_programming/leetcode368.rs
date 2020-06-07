// https://leetcode.com/problems/largest-divisible-subset/
// Runtime: 20 ms
// Memory Usage: 2.1 MB
pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let n = nums.len();
    let mut index = (0..n).collect::<Vec<_>>();
    let mut size = vec![1; n];
    let mut max_size = 1;
    nums.sort_unstable();
    for i in 0..n {
        for j in 0..i {
            if nums[i] % nums[j] == 0 && size[j] + 1 > size[i] {
                index[i] = j;
                size[i] = size[j] + 1;
                max_size = max_size.max(size[i]);
            }
        }
    }
    let mut res = vec![];
    for i in 0..n {
        if size[i] == max_size {
            let mut j = i;
            while index[j] != j {
                res.push(nums[j]);
                j = index[j];
            }
            res.push(nums[j]);
            break;
        }
    }
    res.reverse();
    res
}
// math dynamic_programming
#[test]
fn test2_368() {
    assert_eq!(largest_divisible_subset(vec![1, 2, 3]), vec![1, 2]);
    assert_eq!(largest_divisible_subset(vec![1, 2, 4, 8]), vec![1, 2, 4, 8]);
}
