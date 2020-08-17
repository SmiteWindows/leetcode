// https://leetcode.com/problems/stone-game-iv/
// Runtime: 4 ms
// Memory Usage: 2.5 MB
pub fn winner_square_game(n: i32) -> bool {
    let n = n as usize;
    let mut cache = vec![0_u8; n + 1];
    cache[0] = 2;
    dp(&mut cache, n)
}

fn dp(cache: &mut Vec<u8>, n: usize) -> bool {
    if cache[n] != 0 {
        return cache[n] == 1;
    }
    let sqrt = (n as f64).sqrt().floor() as usize;
    for i in (1..=sqrt).rev() {
        if !dp(cache, n - i * i) {
            cache[n] = 1;
            return true;
        }
    }
    cache[n] = 2;
    false
}
// dynamic_programming
#[test]
fn test1_1510() {
    assert_eq!(winner_square_game(1), true);
    assert_eq!(winner_square_game(2), false);
    assert_eq!(winner_square_game(4), true);
    assert_eq!(winner_square_game(7), false);
    assert_eq!(winner_square_game(17), false);
}
