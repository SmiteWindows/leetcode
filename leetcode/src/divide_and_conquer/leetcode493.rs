// https://leetcode.com/problems/reverse-pairs/
// Runtime: 64 ms
// Memory Usage: 2.6 MB
pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let n = nums.len();
    let mut temp = vec![0; n];
    merge_sort(0, n, &mut nums, &mut temp) as i32
}

fn merge_sort(start: usize, end: usize, nums: &mut Vec<i32>, temp: &mut Vec<i32>) -> usize {
    if start + 1 >= end {
        return 0;
    }
    let mid = start + (end - start) / 2;
    let mut res = 0;
    res += merge_sort(start, mid, nums, temp);
    res += merge_sort(mid, end, nums, temp);
    let mut i = start;
    let mut j = mid;
    while i < mid {
        while j < end && nums[i] as i64 > 2 * nums[j] as i64 {
            j += 1;
        }
        res += j - mid;
        i += 1;
    }
    let mut k = start;
    let mut i = start;
    let mut j = mid;
    while i < mid || j < end {
        if i == mid {
            temp[k] = nums[j];
            k += 1;
            j += 1;
            continue;
        }
        if j == end {
            temp[k] = nums[i];
            k += 1;
            i += 1;
            continue;
        }
        if nums[i] < nums[j] {
            temp[k] = nums[i];
            i += 1;
        } else {
            temp[k] = nums[j];
            j += 1;
        }
        k += 1;
    }
    nums[start..end].clone_from_slice(&temp[start..end]);
    res
}
// divide_and_conquer binary_indexed_tree segment_tree binary_search sort
#[test]
fn test2_493() {
    assert_eq!(reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
    assert_eq!(reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
}
