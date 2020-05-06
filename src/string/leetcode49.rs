// https://leetcode.com/problems/group-anagrams/
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    todo!()
}
// hash_table string
#[test]
#[ignore]
fn test2_49() {
    assert_eq!(
        group_anagrams(vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat")
        ]),
        vec![
            vec![
                String::from("ate"),
                String::from("eat"),
                String::from("tea")
            ],
            vec![String::from("nat"), String::from("tan")],
            vec![String::from("bat")]
        ]
    );
}
