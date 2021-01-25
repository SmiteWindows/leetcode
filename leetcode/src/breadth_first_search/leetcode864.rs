// https://leetcode-cn.com/problems/shortest-path-to-get-all-keys/
pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
    todo!()
}
// heap breadth_first_search
#[test]
#[ignore]
fn test2_864() {
    use leetcode_prelude::vec_string;
    assert_eq!(
        shortest_path_all_keys(vec_string!["@.a.#", "###.#", "b.A.B"]),
        8
    );
    assert_eq!(
        shortest_path_all_keys(vec_string!["@..aA", "..B#.", "....b"]),
        6
    );
}
