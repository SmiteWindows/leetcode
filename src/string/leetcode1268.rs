// https://leetcode.com/problems/search-suggestions-system/
pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    todo!()
}
// string
#[test]
#[ignore]
fn test1_1268() {
    assert_eq!(
        suggested_products(
            vec![
                String::from("mobile"),
                String::from("mouse"),
                String::from("moneypot"),
                String::from("monitor"),
                String::from("mousepad")
            ],
            String::from("mouse")
        ),
        vec![
            vec![
                String::from("mobile"),
                String::from("moneypot"),
                String::from("monitor")
            ],
            vec![
                String::from("mobile"),
                String::from("moneypot"),
                String::from("monitor")
            ],
            vec![String::from("mouse"), String::from("mousepad")],
            vec![String::from("mouse"), String::from("mousepad")],
            vec![String::from("mouse"), String::from("mousepad")]
        ]
    );
    assert_eq!(
        suggested_products(vec![String::from("havana")], String::from("havana")),
        vec![
            vec![String::from("havana")],
            vec![String::from("havana")],
            vec![String::from("havana")],
            vec![String::from("havana")],
            vec![String::from("havana")],
            vec![String::from("havana")]
        ]
    );
    assert_eq!(
        suggested_products(
            vec![
                String::from("bags"),
                String::from("baggage"),
                String::from("banner"),
                String::from("box"),
                String::from("cloths")
            ],
            String::from("bags")
        ),
        vec![
            vec![
                String::from("baggage"),
                String::from("bags"),
                String::from("banner")
            ],
            vec![
                String::from("baggage"),
                String::from("bags"),
                String::from("banner")
            ],
            vec![String::from("baggage"), String::from("bags")],
            vec![String::from("bags")]
        ]
    );
    assert_eq!(
        suggested_products(vec![String::from("havana")], String::from("tatiana")),
        vec![
            vec![String::new()],
            vec![String::new()],
            vec![String::new()],
            vec![String::new()],
            vec![String::new()],
            vec![String::new()],
            vec![String::new()]
        ]
    );
}
