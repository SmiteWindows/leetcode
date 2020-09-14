// https://leetcode-cn.com/problems/count-unhappy-friends/
pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// array
#[test]
#[ignore]
fn test1_1583() {
    use leetcode_prelude::vec2;
    assert_eq!(
        unhappy_friends(
            4,
            vec2![[1, 2, 3], [3, 2, 0], [3, 1, 0], [1, 2, 0]],
            vec2![[0, 1], [2, 3]]
        ),
        2
    );
    assert_eq!(unhappy_friends(2, vec2![[1], [0]], vec2![[1, 0]]), 0);
    assert_eq!(
        unhappy_friends(
            4,
            vec2![[1, 3, 2], [2, 3, 0], [1, 3, 0], [0, 2, 1]],
            vec2![[1, 3], [0, 2]]
        ),
        4
    );
}
