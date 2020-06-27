// https://leetcode.com/problems/perfect-squares/
// Runtime: 16 ms
// Memory Usage: 2.1 MB
pub fn num_squares(n: i32) -> i32 {
    // let mut n = n;
    // if is_square(n as f64) {
    //     return 1;
    // }
    // while n.trailing_zeros() >= 2 {
    //     n >>= 2;
    // }
    // if n & 7 == 7 {
    //     return 4;
    // }
    // let mut i = 1;
    // while i * i <= n {
    //     if is_square((n - i * i) as f64) {
    //         return 2;
    //     }
    //     i += 1;
    // }
    // 3
    let n = n as usize;
    let largest_root = (n as f32).sqrt() as usize;
    let mut dp = vec![0; n + 1];
    for i in 1..=largest_root {
        dp[i * i] = 1;
    }
    let mut last_root = 1;
    for i in 2..=n {
        if dp[i] == 0 {
            dp[i] = (1..=last_root)
                .map(|j| dp[j * j] + dp[i - j * j])
                .min()
                .unwrap();
        } else {
            last_root += 1;
        }
    }
    dp[dp.len() - 1]
}

// fn is_square(n: f64) -> bool {
//     let sq = n.sqrt();
//     (n - sq * sq).abs() <= f64::EPSILON
// }
// math breadth_first_search dynamic_programming
#[test]
fn test1_279() {
    assert_eq!(num_squares(11), 3);
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
}
