// https://leetcode.com/problems/stone-game-iii/
pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
    todo!()
}
// dynamic_programming
#[test]
#[ignore]
fn test1_1406() {
    assert_eq!(stone_game_iii(vec![1, 2, 3, 7]), String::from("Bob"));
    assert_eq!(stone_game_iii(vec![1, 2, 3, -9]), String::from("Alice"));
    assert_eq!(stone_game_iii(vec![1, 2, 3, 6]), String::from("Tie"));
    assert_eq!(
        stone_game_iii(vec![1, 2, 3, -1, -2, -3, 7]),
        String::from("Alice")
    );
    assert_eq!(stone_game_iii(vec![-1, -2, -3]), String::from("Tie"));
}
