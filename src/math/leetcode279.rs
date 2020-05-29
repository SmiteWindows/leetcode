// https://leetcode.com/problems/perfect-squares/
pub fn num_squares(n: i32) -> i32 {
    let mut n = n;
    while n &3 == 0 {
        n >>=2;
    }
    if n &7 == 7 {
        return 4;
    }
    if is_square(n as f64) {
        return 1;
    }
    let mut i = 1;
    while i * i <= n {
        if is_square((n - i * i) as f64) {
            return 2;
        }
        i += 1;
    }
    3
}

fn is_square(n: f64) -> bool {
    let sq = n.sqrt();
    n == sq * sq
}
// math breadth_first_search dynamic_programming
#[test]
#[ignore]
fn test1_279() {
    assert_eq!(num_squares(11), 3);
    assert_eq!(num_squares(12), 3);
    assert_eq!(num_squares(13), 2);
}
