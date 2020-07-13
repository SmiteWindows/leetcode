// https://leetcode.com/problems/stone-game-iv/
pub fn winner_square_game(n: i32) -> bool {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1510() {
    assert_eq!(winner_square_game(1), true);
    assert_eq!(winner_square_game(2), false);
    assert_eq!(winner_square_game(4), true);
    assert_eq!(winner_square_game(7), false);
    assert_eq!(winner_square_game(17), false);
}
