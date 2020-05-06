// https://leetcode.com/problems/minimum-index-sum-of-two-lists/
pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_599() {
    assert_eq!(
        find_restaurant(
            vec![
                String::from("Shogun"),
                String::from("Tapioca Express"),
                String::from("Burger King"),
                String::from("KFC")
            ],
            vec![
                String::from("Piatti"),
                String::from("The Grill at Torrey Pines"),
                String::from("Hungry Hunter Steakhouse"),
                String::from("Shogun")
            ]
        ),
        vec![String::from("Shogun")]
    );
    assert_eq!(
        find_restaurant(
            vec![
                String::from("Shogun"),
                String::from("Tapioca Express"),
                String::from("Burger King"),
                String::from("KFC")
            ],
            vec![
                String::from("KFC"),
                String::from("Shogun"),
                String::from("Burger King")
            ]
        ),
        vec![String::from("Shogun")]
    );
}
