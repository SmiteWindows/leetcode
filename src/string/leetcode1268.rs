// https://leetcode.com/problems/search-suggestions-system/
// Runtime: 8 ms
// Memory Usage: 4.5 MB
pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    let mut products = products;
    products.sort();
    let n = search_word.len();
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        let w = &search_word[0..i + 1];
        products = products.into_iter().filter(|p| p.starts_with(w)).collect();
        res.push(
            products
                .iter()
                .take(3)
                .map(|p| p.to_string())
                .collect::<Vec<_>>(),
        );
    }
    res
}
// string
#[test]
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
            vec![] as Vec<String>,
            vec![] as Vec<String>,
            vec![] as Vec<String>,
            vec![] as Vec<String>,
            vec![] as Vec<String>,
            vec![] as Vec<String>,
            vec![] as Vec<String>
        ]
    );
}
