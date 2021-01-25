// https://leetcode-cn.com/problems/number-of-recent-calls/
// Runtime: 36 ms
// Memory Usage: 5.4 MB
use std::collections::VecDeque;
struct RecentCounter {
    queue: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        // while let Some(p) = self.queue.pop_front() {
        //     if p >= t - 3000 {
        //         self.queue.push_front(p);
        //         break;
        //     }
        // }
        while let Some(a) = self.queue.front() {
            if *a < t - 3000 {
                self.queue.pop_front();
            } else {
                break;
            }
        }
        self.queue.len() as i32
    }
}
/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
#[test]
fn test1_933() {
    let mut obj = RecentCounter::new();
    assert_eq!(obj.ping(1), 1);
    assert_eq!(obj.ping(100), 2);
    assert_eq!(obj.ping(3001), 3);
    assert_eq!(obj.ping(3002), 3);
}
