// https://leetcode-cn.com/problems/design-browser-history/
// Runtime: 44 ms
// Memory Usage: 3.7 MB
struct BrowserHistory {
    backward: Vec<String>,
    forward: Vec<String>,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            backward: vec![homepage],
            forward: vec![],
        }
    }

    fn visit(&mut self, url: String) {
        self.backward.push(url);
        self.forward = Vec::new();
    }

    fn back(&mut self, steps: i32) -> String {
        for _ in 0..steps {
            if self.backward.len() > 1 {
                self.forward.push(self.backward.pop().unwrap());
            }
        }
        self.backward.last().unwrap().to_string()
    }

    fn forward(&mut self, steps: i32) -> String {
        for _ in 0..steps {
            if !self.forward.is_empty() {
                self.backward.push(self.forward.pop().unwrap());
            }
        }
        self.backward.last().unwrap().to_string()
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
fn test1_1472() {
    let mut obj = BrowserHistory::new("leetcode.com".to_string());
    obj.visit("google.com".to_string()); // You are in "leetcode.com". Visit "google.com"
    obj.visit("facebook.com".to_string()); // You are in "google.com". Visit "facebook.com"
    obj.visit("youtube.com".to_string()); // You are in "facebook.com". Visit "youtube.com"
    assert_eq!(obj.back(1), "facebook.com".to_string()); // You are in "youtube.com", move back to "facebook.com" return "facebook.com"
    assert_eq!(obj.back(1), "google.com".to_string()); // You are in "facebook.com", move back to "google.com" return "google.com"
    assert_eq!(obj.forward(1), "facebook.com".to_string()); // You are in "google.com", move forward to "facebook.com" return "facebook.com"
    obj.visit("linkedin.com".to_string()); // You are in "facebook.com". Visit "linkedin.com"
    assert_eq!(obj.forward(2), "linkedin.com".to_string()); // You are in "linkedin.com", you cannot move forward any steps.
    assert_eq!(obj.back(2), "google.com".to_string()); // You are in "linkedin.com", move back two steps to "facebook.com" then to "google.com". return "google.com"
    assert_eq!(obj.back(7), "leetcode.com".to_string()); // You are in "google.com", you can move back only one step to "leetcode.com". return "leetcode.com"
}
