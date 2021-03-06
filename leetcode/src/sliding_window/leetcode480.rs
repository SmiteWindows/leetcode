// https://leetcode-cn.com/problems/sliding-window-median/
// Runtime: 8 ms
// Memory Usage: 2.2 MB
pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let k = k as usize;
    let n = nums.len();
    let mut w = Vec::new();
    for &num in nums.iter().take(k) {
        w.push(num);
    }
    w.sort_unstable();
    let mut res = vec![median(&w, k)];
    for i in k..n {
        let pos = w.binary_search(&nums[i - k]).unwrap_or_else(|e| e);
        w.remove(pos);
        let pos = w.binary_search(&nums[i]).unwrap_or_else(|e| e);
        w.insert(pos, nums[i]);
        res.push(median(&w, k));
    }
    res
}

fn median(v: &[i32], k: usize) -> f64 {
    (v[(k - 1) / 2] as f64 + v[k / 2] as f64) / 2.0
}
// sliding_window
#[test]
fn test1_480() {
    assert_eq!(
        median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![1.0, -1.0, -1.0, 3.0, 5.0, 6.0]
    );
}
