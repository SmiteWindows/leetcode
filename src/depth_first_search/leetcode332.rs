// https://leetcode.com/problems/reconstruct-itinerary/
pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    todo!()
}
// graph depth_first_search
#[test]
#[ignore]
fn test2_332() {
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