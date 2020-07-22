// https://leetcode.com/problems/design-twitter/
// Runtime: 4 ms
// Memory Usage: 9.4 MB
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};
struct Twitter {
    time: usize,
    users: HashMap<i32, HashSet<i32>>,
    tweets: HashMap<i32, Vec<(Reverse<usize>, i32)>>,
    limit: usize,
}

impl Twitter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            time: 0,
            users: HashMap::new(),
            tweets: HashMap::new(),
            limit: 10,
        }
    }

    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.time += 1;
        let tweet = (Reverse(self.time), tweet_id);
        self.tweets.entry(user_id).or_default().push(tweet);
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut pq = BinaryHeap::with_capacity(self.limit + 1);
        let mut res = vec![];
        let followers = self.users.entry(user_id).or_default();
        followers.insert(user_id);
        for &user in followers.iter() {
            let tweets = self.tweets.entry(user).or_default();
            for tweet in tweets {
                pq.push(*tweet);
                if pq.len() > self.limit {
                    pq.pop();
                }
            }
        }
        while !pq.is_empty() {
            let earliest = pq.pop().expect("exist");
            res.push(earliest.1);
        }
        res.reverse();
        res
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .or_default()
            .remove(&followee_id);
    }
}
/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
// design heap hash_table
#[test]
fn test3_355() {
    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5);
    assert_eq!(twitter.get_news_feed(1), vec![5]);
    twitter.follow(1, 2);
    twitter.post_tweet(2, 6);
    assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
    twitter.unfollow(1, 2);
    assert_eq!(twitter.get_news_feed(1), vec![5]);
}
