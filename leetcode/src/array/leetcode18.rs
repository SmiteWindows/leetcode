// https://leetcode-cn.com/problems/4sum/
// Runtime: 0 ms
// Memory Usage: 2 MB
use std::cmp::Ordering::{Equal, Greater, Less};
pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut res = Vec::new();
    nums.sort_unstable();
    let mut i = 0;

    while i + 3 < n {
        if i > 0 && nums[i - 1] == nums[i] {
            i += 1;
            continue;
        }

        if nums[i] + nums[i + 1] + nums[i + 2] + nums[i + 3] > target {
            break;
        }

        if nums[n - 3] + nums[n - 2] + nums[n - 1] + nums[i] < target {
            i += 1;
            continue;
        }

        let mut j = i + 1;

        while j + 2 < n {
            if j > i + 1 && nums[j - 1] == nums[j] {
                j += 1;
                continue;
            }

            if nums[i] + nums[j] + nums[j + 1] + nums[j + 2] > target {
                break;
            }

            if nums[n - 2] + nums[n - 1] + nums[i] + nums[j] < target {
                j += 1;
                continue;
            }

            let sum2 = nums[i] + nums[j];
            let mut left = j + 1;
            let mut right = n - 1;

            while left < right {
                let sum4 = sum2 + nums[left] + nums[right];
                match sum4.cmp(&target) {
                    Less => {
                        left += 1;
                    }
                    Greater => {
                        right -= 1;
                    }
                    Equal => {
                        res.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < right && nums[left - 1] == nums[left] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    }
                }
            }
            j += 1
        }
        i += 1
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
