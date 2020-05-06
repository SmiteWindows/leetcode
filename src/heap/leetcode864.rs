// https://leetcode.com/problems/shortest-path-to-get-all-keys/
pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
    todo!()
}
// heap breadth_first_search
#[test]
#[ignore]
fn test1_864() {
    assert_eq!(
        shortest_path_all_keys(vec![
            String::from("@.a.#"),
            String::from("###.#"),
            String::from("b.A.B")
        ]),
        8
    );
    assert_eq!(
        shortest_path_all_keys(vec![
            String::from("@..aA"),
            String::from("..B#."),
            String::from("....b")
        ]),
        6
    );
}
