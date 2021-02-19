// https://leetcode-cn.com/problems/search-suggestions-system/
// Runtime: 8 ms
// Memory Usage: 4.5 MB
pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
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
    use leetcode_prelude::{vec2_string, vec_string};
    assert_eq!(
        suggested_products(
            vec_string!["mobile", "mouse", "moneypot", "monitor", "mousepad"],
            "mouse".to_string()
        ),
        vec2_string![
            ["mobile", "moneypot", "monitor"],
            ["mobile", "moneypot", "monitor"],
            ["mouse", "mousepad"],
            ["mouse", "mousepad"],
            ["mouse", "mousepad"]
        ]
    );
    assert_eq!(
        suggested_products(vec_string!["havana"], "havana".to_string()),
        vec2_string![
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"]
        ]
    );
    assert_eq!(
        suggested_products(
            vec_string!["bags", "baggage", "banner", "box", "cloths"],
            "bags".to_string()
        ),
        vec2_string![
            ["baggage", "bags", "banner"],
            ["baggage", "bags", "banner"],
            ["baggage", "bags"],
            ["bags"]
        ]
    );
    assert_eq!(
        suggested_products(vec_string!["havana"], "tatiana".to_string()),
        vec2_string![[], [], [], [], [], [], []]
    );
}
