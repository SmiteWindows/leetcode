// https://leetcode.com/problems/design-twitter/
struct Twitter {}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        todo!()
    }

    /** Compose a new tweet. */
    fn post_tweet(&self, user_id: i32, tweet_id: i32) {
        todo!()
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        todo!()
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&self, follower_id: i32, followee_id: i32) {
        todo!()
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&self, follower_id: i32, followee_id: i32) {
        todo!()
    }
}
