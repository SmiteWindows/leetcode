// https://leetcode.com/problems/design-browser-history/
struct BrowserHistory {}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        todo!()
    }

    fn visit(&self, url: String) {
        todo!()
    }

    fn back(&self, steps: i32) -> String {
        todo!()
    }

    fn forward(&self, steps: i32) -> String {
        todo!()
    }
}
/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */
// design
#[test]
#[ignore]
fn test1_1472() {
    // let obj = BrowserHistory::new("leetcode.com".to_string());
    // browserHistory.visit("google.com"); // You are in "leetcode.com". Visit "google.com"
    // browserHistory.visit("facebook.com"); // You are in "google.com". Visit "facebook.com"
    // browserHistory.visit("youtube.com"); // You are in "facebook.com". Visit "youtube.com"
    // assert_eq!(browserHistory.back(1), "facebook.com".to_string()); // You are in "youtube.com", move back to "facebook.com" return "facebook.com"
    // assert_eq!(browserHistory.back(1), "google.com".to_string()); // You are in "facebook.com", move back to "google.com" return "google.com"
    // assert_eq!(browserHistory.forward(1), "facebook.com".to_string()); // You are in "google.com", move forward to "facebook.com" return "facebook.com"
    // browserHistory.visit("linkedin.com"); // You are in "facebook.com". Visit "linkedin.com"
    // assert_eq!(browserHistory.forward(2), "".to_string()); // You are in "linkedin.com", you cannot move forward any steps.
    // assert_eq!(browserHistory.back(2), "google.com".to_string()); // You are in "linkedin.com", move back two steps to "facebook.com" then to "google.com". return "google.com"
    // assert_eq!(browserHistory.back(7), "leetcode.com".to_string()); // You are in "google.com", you can move back only one step to "leetcode.com". return "leetcode.com"
}
