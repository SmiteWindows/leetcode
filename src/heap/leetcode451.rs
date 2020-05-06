// https://leetcode.com/problems/sort-characters-by-frequency/
pub fn frequency_sort(s: String) -> String {
    todo!()
}
// hash_table heap
#[test]
#[ignore]
fn test2_451() {
    assert_eq!(frequency_sort(String::from("tree")), String::from("eert"));
    assert_eq!(
        frequency_sort(String::from("cccaaa")),
        String::from("cccaaa")
    );
    assert_eq!(frequency_sort(String::from("Aabb")), String::from("bbAa"));
}
