// https://leetcode.com/problems/accounts-merge/
pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    todo!()
}
// depth_first_search union_find
#[test]
#[ignore]
fn test1_721() {
    assert_eq!(
        accounts_merge(vec![
            vec![
                String::from("John"),
                String::from("johnsmith@mail.com"),
                String::from("john00@mail.com")
            ],
            vec![String::from("John"), String::from("johnnybravo@mail.com")],
            vec![
                String::from("John"),
                String::from("johnsmith@mail.com"),
                String::from("john_newyork@mail.com")
            ],
            vec![String::from("Mary"), String::from("mary@mail.com")],
        ]),
        vec![
            vec![
                String::from("John"),
                String::from("john00@mail.com"),
                String::from("john_newyork@mail.com"),
                String::from("johnsmith@mail.com")
            ],
            vec![String::from("John"), String::from("johnnybravo@mail.com")],
            vec![String::from("Mary"), String::from("mary@mail.com")]
        ]
    );
}
