// https://leetcode.com/problems/display-table-of-food-orders-in-a-restaurant/
pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    todo!()
}
// hash_table
#[test]
#[ignore]
fn test1_1418() {
    assert_eq!(
        display_table(vec![
            vec!["David", "3", "Ceviche"],
            vec!["Corina", "10", "Beef Burrito"],
            vec!["David", "3", "Fried Chicken"],
            vec!["Carla", "5", "Water"],
            vec!["Carla", "5", "Ceviche"],
            vec!["Rous", "3", "Ceviche"]
        ]),
        vec![
            vec!["Table", "Beef Burrito", "Ceviche", "Fried Chicken", "Water"],
            vec!["3", "0", "2", "1", "0"],
            vec!["5", "0", "1", "0", "1"],
            vec!["10", "1", "0", "0", "0"]
        ]
    );
    assert_eq!(
        display_table(vec![
            vec!["James", "12", "Fried Chicken"],
            vec!["Ratesh", "12", "Fried Chicken"],
            vec!["Amadeus", "12", "Fried Chicken"],
            vec!["Adam", "1", "Canadian Waffles"],
            vec!["Brianna", "1", "Canadian Waffles"]
        ]),
        vec![
            vec!["Table", "Canadian Waffles", "Fried Chicken"],
            vec!["1", "2", "0"],
            vec!["12", "0", "3"]
        ]
    );
    assert_eq!(
        display_table(vec![
            vec!["Laura", "2", "Bean Burrito"],
            vec!["Jhon", "2", "Beef Burrito"],
            vec!["Melissa", "2", "Soda"]
        ]),
        vec![
            vec!["Table", "Bean Burrito", "Beef Burrito", "Soda"],
            vec!["2", "1", "1", "1"]
        ]
    );
}
