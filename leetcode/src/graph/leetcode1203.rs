// https://leetcode.com/problems/sort-items-by-groups-respecting-dependencies/
pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}
// depth_first_search topological_sort graph
#[test]
#[ignore]
fn test3_1203() {
    assert_eq!(
        sort_items(
            8,
            2,
            vec![-1, -1, 1, 0, 0, 1, 0, -1],
            vec![
                vec![],
                vec![6],
                vec![5],
                vec![6],
                vec![3, 6],
                vec![],
                vec![],
                vec![]
            ]
        ),
        vec![6, 3, 4, 1, 5, 2, 0, 7]
    );
    assert_eq!(
        sort_items(
            8,
            2,
            vec![-1, -1, 1, 0, 0, 1, 0, -1],
            vec![
                vec![],
                vec![6],
                vec![5],
                vec![6],
                vec![3],
                vec![],
                vec![4],
                vec![]
            ]
        ),
        vec![]
    );
}
