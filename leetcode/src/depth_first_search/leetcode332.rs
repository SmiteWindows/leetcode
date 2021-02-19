// https://leetcode-cn.com/problems/reconstruct-itinerary/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut res = Vec::new();
    let mut g: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new();
    for ticket in tickets {
        g.entry(ticket[0].clone())
            .or_default()
            .push(Reverse(ticket[1].clone()));
    }
    let mut stack = vec!["JFK".to_string()];
    while !stack.is_empty() {
        while g.contains_key(stack.last().unwrap())
            && !g.get(stack.last().unwrap()).unwrap().is_empty()
        {
            let airports = g.get_mut(stack.last().unwrap()).unwrap();
            let airport = airports.pop().unwrap().0;
            stack.push(airport);
        }
        res.insert(0, stack.pop().unwrap());
    }
    res
}
// graph depth_first_search
#[test]
fn test2_332() {
    use leetcode_prelude::{vec2_string, vec_string};
    assert_eq!(
        find_itinerary(vec2_string![
            ["MUC", "LHR"],
            ["JFK", "MUC"],
            ["SFO", "SJC"],
            ["LHR", "SFO"]
        ]),
        vec_string!["JFK", "MUC", "LHR", "SFO", "SJC"]
    );
    assert_eq!(
        find_itinerary(vec2_string![
            ["JFK", "SFO"],
            ["JFK", "ATL"],
            ["SFO", "ATL"],
            ["ATL", "JFK"],
            ["ATL", "SFO"]
        ]),
        vec_string!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
    );
}
