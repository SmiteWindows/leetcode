// https://leetcode.com/problems/satisfiability-of-equality-equations/
pub fn equations_possible(equations: Vec<String>) -> bool {
    todo!()
}
// union_find graph
#[test]
#[ignore]
fn test2_990() {
    assert_eq!(
        equations_possible(vec![String::from("a==b"), String::from("b!=a")]),
        false
    );
    assert_eq!(
        equations_possible(vec![String::from("b==a"), String::from("a==b")]),
        true
    );
    assert_eq!(
        equations_possible(vec![
            String::from("a==b"),
            String::from("b==c"),
            String::from("a==c")
        ]),
        true
    );
    assert_eq!(
        equations_possible(vec![
            String::from("a==b"),
            String::from("b!=c"),
            String::from("c==a")
        ]),
        false
    );
    assert_eq!(
        equations_possible(vec![
            String::from("c==c"),
            String::from("b==d"),
            String::from("x!=z")
        ]),
        true
    );
}
