// https://leetcode-cn.com/problems/number-of-ways-where-square-of-number-is-equal-to-product-of-two-numbers/
// Runtime: 256 ms
// Memory Usage: 2.2 MB
use std::collections::HashMap;
pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let nums1: Vec<i64> = nums1.into_iter().map(|x| x as i64).collect();
    let nums2: Vec<i64> = nums2.into_iter().map(|x| x as i64).collect();
    let nums1_square: Vec<i64> = nums1.iter().map(|x| x * x).collect();
    let nums2_square: Vec<i64> = nums2.iter().map(|x| x * x).collect();
    let mut res = 0;
    for x in nums1_square {
        res += product(x, &nums2);
    }
    for x in nums2_square {
        res += product(x, &nums1);
    }
    res
}

fn product(x: i64, nums: &[i64]) -> i32 {
    let mut count: HashMap<i64, i32> = HashMap::new();
    let mut res = 0;
    for &y in nums {
        if x % y == 0 {
            if let Some(t) = count.get(&(x / y)) {
                res += t;
            }
        }
        *count.entry(y).or_default() += 1;
    }
    res
}
//hash_table math
#[test]
fn test2_1577() {
    assert_eq!(num_triplets(vec![7, 4], vec![5, 2, 8, 9]), 1);
    assert_eq!(num_triplets(vec![1, 1], vec![1, 1, 1]), 9);
    assert_eq!(num_triplets(vec![7, 7, 8, 3], vec![1, 2, 9, 7]), 2);
    assert_eq!(
        num_triplets(vec![4, 7, 9, 11, 23], vec![3, 5, 1024, 12, 18]),
        0
    );
}
