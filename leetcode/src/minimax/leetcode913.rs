// https://leetcode-cn.com/problems/cat-and-mouse/
pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
    todo!()
}
// breadth_first_search minimax
#[test]
#[ignore]
fn test1_913() {
    assert_eq!(
        cat_mouse_game(vec![
            vec![2, 5],
            vec![3],
            vec![0, 4, 5],
            vec![1, 4, 5],
            vec![2, 3],
            vec![0, 2, 3]
        ]),
        0
    );
}
