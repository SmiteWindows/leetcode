// https://leetcode-cn.com/problems/maximum-candies-you-can-get-from-boxes/
pub fn max_candies(
    status: Vec<i32>,
    candies: Vec<i32>,
    keys: Vec<Vec<i32>>,
    contained_boxes: Vec<Vec<i32>>,
    initial_boxes: Vec<i32>,
) -> i32 {
    todo!()
}
// breadth_first_search
#[test]
#[ignore]
fn test1_1298() {
    use leetcode_prelude::vec2;
    assert_eq!(
        max_candies(
            vec![1, 0, 1, 0],
            vec![7, 5, 4, 100],
            vec2![[], [], [1], []],
            vec2![[1, 2], [3], [], []],
            vec![0]
        ),
        16
    );
    assert_eq!(
        max_candies(
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1],
            vec2![[1, 2, 3, 4, 5], [], [], [], [], []],
            vec2![[1, 2, 3, 4, 5], [], [], [], [], []],
            vec![0]
        ),
        6
    );
    assert_eq!(
        max_candies(
            vec![1, 1, 1],
            vec![100, 1, 100],
            vec2![[], [0, 2], []],
            vec2![[], [], []],
            vec![1]
        ),
        1
    );
    assert_eq!(
        max_candies(vec![1], vec![100], vec2![[]], vec2![[]], vec![]),
        0
    );
    assert_eq!(
        max_candies(
            vec![1, 1, 1],
            vec![2, 3, 2],
            vec2![[], [], []],
            vec2![[], [], []],
            vec![2, 1, 0]
        ),
        7
    );
}
