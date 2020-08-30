// https://leetcode-cn.com/problems/display-table-of-food-orders-in-a-restaurant/
// Runtime: 120 ms
// Memory Usage: 18.6 MB
use std::collections::{BTreeSet, HashMap};
pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut tables = BTreeSet::new();
    let mut foods: BTreeSet<&str> = BTreeSet::new();
    let mut counts: HashMap<i32, HashMap<&str, usize>> = HashMap::new();
    for order in &orders {
        let table = order[1].parse::<i32>().unwrap();
        let food = &order[2];
        tables.insert(table);
        foods.insert(food);
        *counts.entry(table).or_default().entry(food).or_default() += 1;
    }
    let mut res = vec![vec!["Table".to_string()]];
    for food in foods.iter() {
        res[0].push((*food).to_string());
    }
    for table in tables {
        let mut row = vec![table.to_string()];
        for food in &foods {
            if let Some(counts) = counts.get(&table) {
                if let Some(count) = counts.get(food) {
                    row.push(count.to_string());
                } else {
                    row.push("0".to_string());
                }
            } else {
                row.push("0".to_string());
            }
        }
        res.push(row);
    }
    res
}
// hash_table
#[test]
fn test1_1418() {
    assert_eq!(
        display_table(vec![
            vec![
                String::from("David"),
                String::from("3"),
                String::from("Ceviche")
            ],
            vec![
                String::from("Corina"),
                String::from("10"),
                String::from("Beef Burrito")
            ],
            vec![
                String::from("David"),
                String::from("3"),
                String::from("Fried Chicken")
            ],
            vec![
                String::from("Carla"),
                String::from("5"),
                String::from("Water")
            ],
            vec![
                String::from("Carla"),
                String::from("5"),
                String::from("Ceviche")
            ],
            vec![
                String::from("Rous"),
                String::from("3"),
                String::from("Ceviche")
            ]
        ]),
        vec![
            vec![
                String::from("Table"),
                String::from("Beef Burrito"),
                String::from("Ceviche"),
                String::from("Fried Chicken"),
                String::from("Water")
            ],
            vec![
                String::from("3"),
                String::from("0"),
                String::from("2"),
                String::from("1"),
                String::from("0")
            ],
            vec![
                String::from("5"),
                String::from("0"),
                String::from("1"),
                String::from("0"),
                String::from("1")
            ],
            vec![
                String::from("10"),
                String::from("1"),
                String::from("0"),
                String::from("0"),
                String::from("0")
            ]
        ]
    );
    assert_eq!(
        display_table(vec![
            vec![
                String::from("James"),
                String::from("12"),
                String::from("Fried Chicken")
            ],
            vec![
                String::from("Ratesh"),
                String::from("12"),
                String::from("Fried Chicken")
            ],
            vec![
                String::from("Amadeus"),
                String::from("12"),
                String::from("Fried Chicken")
            ],
            vec![
                String::from("Adam"),
                String::from("1"),
                String::from("Canadian Waffles")
            ],
            vec![
                String::from("Brianna"),
                String::from("1"),
                String::from("Canadian Waffles")
            ]
        ]),
        vec![
            vec![
                String::from("Table"),
                String::from("Canadian Waffles"),
                String::from("Fried Chicken")
            ],
            vec![String::from("1"), String::from("2"), String::from("0")],
            vec![String::from("12"), String::from("0"), String::from("3")]
        ]
    );
    assert_eq!(
        display_table(vec![
            vec![
                String::from("Laura"),
                String::from("2"),
                String::from("Bean Burrito")
            ],
            vec![
                String::from("Jhon"),
                String::from("2"),
                String::from("Beef Burrito")
            ],
            vec![
                String::from("Melissa"),
                String::from("2"),
                String::from("Soda")
            ]
        ]),
        vec![
            vec![
                String::from("Table"),
                String::from("Bean Burrito"),
                String::from("Beef Burrito"),
                String::from("Soda")
            ],
            vec![
                String::from("2"),
                String::from("1"),
                String::from("1"),
                String::from("1")
            ]
        ]
    );
}
