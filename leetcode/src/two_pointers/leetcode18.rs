// https://leetcode-cn.com/problems/4sum/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut res = Vec::new();
    nums.sort_unstable();
    let mut index = 0;
    while index + 3 < n {
        if index > 0 && nums[index - 1] == nums[index] {
            index += 1;
            continue;
        }
        if nums[index] + nums[index + 1] + nums[index + 2] + nums[index + 3] > target {
            break;
        }
        if nums[n - 3] + nums[n - 2] + nums[n - 1] + nums[index] < target {
            index += 1;
            continue;
        }
        let mut j = index + 1;
        while j + 2 < n {
            if j > index + 1 && nums[j - 1] == nums[j] {
                j += 1;
                continue;
            }
            if nums[index] + nums[j] + nums[j + 1] + nums[j + 2] > target {
                break;
            }
            if nums[n - 2] + nums[n - 1] + nums[index] + nums[j] < target {
                j += 1;
                continue;
            }
            let sum2 = nums[index] + nums[j];
            let mut l = j + 1;
            let mut r = n - 1;
            while l < r {
                let sum4 = sum2 + nums[l] + nums[r];
                match sum4.cmp(&target) {
                    Less => {
                        l += 1;
                    }
                    Greater => {
                        r -= 1;
                    }
                    Equal => {
                        res.push(vec![nums[index], nums[j], nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        while l < r && nums[l - 1] == nums[l] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    }
                }
            }
            j += 1
        }
        index += 1
    }
    res
}
// array two_pointers hash_table
#[test]
fn test1_18() {
    use leetcode_prelude::{assert_eq_sorted, vec2};
    assert_eq_sorted!(
        four_sum(vec![1, 0, -1, 0, -2, 2], 0),
        vec2![[-1, 0, 0, 1], [-2, -1, 1, 2], [-2, 0, 0, 2]]
    );
}
