// https://leetcode-cn.com/problems/zuma-game/
pub fn find_min_step(board: String, hand: String) -> i32 {
    todo!()
}
// depth_first_search
#[test]
#[ignore]
fn test1_488() {
    assert_eq!(
        find_min_step(String::from("WRRBBW"), String::from("RB")),
        -1
    );
    assert_eq!(
        find_min_step(String::from("WWRRBBWW"), String::from("WRBRW")),
        2
    );
    assert_eq!(find_min_step(String::from("G"), String::from("GGGGG")), 2);
    assert_eq!(
        find_min_step(String::from("RBYYBBRRB"), String::from("YRBGB")),
        3
    );
}
