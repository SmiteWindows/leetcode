// https://leetcode.com/problems/destination-city/
pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1436() {
    assert_eq!(
        dest_city(vec![
            vec![String::from("London"), String::from("New York")],
            vec![String::from("New York"), String::from("Lima")],
            vec![String::from("Lima"), String::from("Sao Paulo")]
        ]),
        String::from("Sao Paulo")
    );
    assert_eq!(
        dest_city(vec![
            vec![String::from("B"), String::from("C")],
            vec![String::from("D"), String::from("B")],
            vec![String::from("C"), String::from("A")]
        ]),
        String::from("A")
    );
    assert_eq!(
        dest_city(vec![vec![String::from("A"), String::from("Z")]]),
        String::from("Z")
    );
}
