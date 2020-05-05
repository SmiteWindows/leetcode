// https://leetcode.com/problems/smallest-sufficient-team/
pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
    todo!()
}
// bit_manipulation dynamic_programming
#[test]
#[ignore]
fn test1_1125() {
    assert_eq!(
        smallest_sufficient_team(
            vec![
                String::from("java"),
                String::from("nodejs"),
                String::from("reactjs")
            ],
            vec![
                vec![String::from("java")],
                vec![String::from("nodejs")],
                vec![String::from("nodejs"), String::from("reactjs")]
            ]
        ),
        vec![0, 2]
    );
    assert_eq!(
        smallest_sufficient_team(
            vec![
                String::from("algorithms"),
                String::from("math"),
                String::from("java"),
                String::from("reactjs"),
                String::from("csharp"),
                String::from("aws")
            ],
            vec![
                vec![
                    String::from("algorithms"),
                    String::from("math"),
                    String::from("java")
                ],
                vec![
                    String::from("algorithms"),
                    String::from("math"),
                    String::from("reactjs")
                ],
                vec![
                    String::from("java"),
                    String::from("csharp"),
                    String::from("aws")
                ],
                vec![String::from("reactjs"), String::from("csharp")],
                vec![String::from("csharp"), String::from("math")],
                vec![String::from("aws"), String::from("java")]
            ]
        ),
        vec![1, 2]
    );
}
