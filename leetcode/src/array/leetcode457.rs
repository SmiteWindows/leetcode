// https://leetcode.com/problems/circular-array-loop/
// Runtime: 0 ms
// Memory Usage: 2.1 MB
pub fn circular_array_loop(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    let n = nums.len();
    for i in 0..n {
        if next(&nums, i) == i {
            nums[i] = 0;
        }
    }
    for i in 0..n {
        if nums[i] == 0 {
            continue;
        }
        let mut slow = i;
        let mut fast = i;
        while nums[slow] * nums[next(&nums, fast)] > 0
            && nums[slow] * nums[next(&nums, next(&nums, fast))] > 0
        {
            slow = next(&nums, slow);
            fast = next(&nums, next(&nums, fast));
            if slow == fast {
                return true;
            }
        }
        let mut j = i;
        let val = nums[i];
        while nums[j] * val > 0 {
            let next = next(&nums, j);
            nums[j] = 0;
            j = next;
        }
    }
    false
}

fn next(nums: &[i32], index: usize) -> usize {
    let n = nums.len();
    let index = index as i32 + nums[index];
    let index = if index < 0 {
        n as i32 + (index % n as i32)
    } else {
        index % n as i32
    };
    (index as usize) % n
}
// array two_pointers
#[test]
fn test2_457() {
    assert_eq!(circular_array_loop(vec![2, -1, 1, 2, 2]), true);
    assert_eq!(circular_array_loop(vec![-1, 2]), false);
    assert_eq!(circular_array_loop(vec![-2, 1, -1, -2, -2]), false);
}
