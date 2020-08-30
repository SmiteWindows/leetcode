// https://leetcode-cn.com/problems/reconstruct-itinerary/
// Runtime: 4 ms
// Memory Usage: 2.1 MB
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut res = vec![];
    let mut g: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new();
    for ticket in tickets {
        g.entry(ticket[0].clone())
            .or_default()
            .push(Reverse(ticket[1].clone()));
    }
    let mut stack = vec![];
    stack.push("JFK".to_string());
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
fn test1_332() {
    assert_eq!(
        find_itinerary(vec![
            vec![String::from("MUC"), String::from("LHR")],
            vec![String::from("JFK"), String::from("MUC")],
            vec![String::from("SFO"), String::from("SJC")],
            vec![String::from("LHR"), String::from("SFO")]
        ]),
        vec![
            String::from("JFK"),
            String::from("MUC"),
            String::from("LHR"),
            String::from("SFO"),
            String::from("SJC")
        ]
    );
    assert_eq!(
        find_itinerary(vec![
            vec![String::from("JFK"), String::from("SFO")],
            vec![String::from("JFK"), String::from("ATL")],
            vec![String::from("SFO"), String::from("ATL")],
            vec![String::from("ATL"), String::from("JFK")],
            vec![String::from("ATL"), String::from("SFO")]
        ]),
        vec![
            String::from("JFK"),
            String::from("ATL"),
            String::from("JFK"),
            String::from("SFO"),
            String::from("ATL"),
            String::from("SFO")
        ]
    );
}
