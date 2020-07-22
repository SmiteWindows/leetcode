// https://leetcode.com/problems/predict-the-winner/
// Runtime: 0 ms
// Memory Usage: 2 MB
pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    let n = nums.len();
    dp(1, 0, n, 0, &nums)
}

fn dp(player: i32, start: usize, end: usize, sum: i32, nums: &[i32]) -> bool {
    if start == end {
        if player > 0 {
            sum >= 0
        } else {
            sum < 0
        }
    } else {
        let mut res = false;
        res |= !dp(-player, start + 1, end, sum + player * nums[start], nums);
        if !res {
            res |= !dp(-player, start, end - 1, sum + player * nums[end - 1], nums);
        }
        res
    }
}
// dynamic_programming minimax
#[test]
fn test2_486() {
    assert_eq!(predict_the_winner(vec![1, 5, 2]), false);
    assert_eq!(predict_the_winner(vec![1, 5, 233, 7]), true);
}
