// https://leetcode-cn.com/problems/beautiful-arrangement/
// Runtime: 32 ms
// Memory Usage: 2.1 MB
pub fn count_arrangement(n: i32) -> i32 {
    let mut res = 0;
    let mut nums = (1..=n).collect();
    let n = n as usize;
    dfs(0, &mut nums, &mut res, n);
    res
}

fn dfs(start: usize, nums: &mut Vec<i32>, res: &mut i32, n: usize) {
    if start == n {
        *res += 1;
    } else {
        for i in start..n {
            if nums[i] % (start as i32 + 1) == 0 || (start as i32 + 1) % nums[i] == 0 {
                nums.swap(start, i);
                dfs(start + 1, nums, res, n);
                nums.swap(start, i);
            }
        }
    }
}
// backtracking
#[test]
fn test1_526() {
    assert_eq!(count_arrangement(2), 2);
}
