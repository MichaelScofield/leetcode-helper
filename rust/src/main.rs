use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

mod helper;

struct Twitter {
    relation: HashMap<i32, HashSet<i32>>,
    tweets: HashMap<i32, Vec<Tweet>>,
}

#[derive(Eq, PartialEq)]
struct Tweet {
    tweet_id: i32,
    tweet_time: Instant,
}

impl PartialOrd for Tweet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let order = other.tweet_time.partial_cmp(&self.tweet_time);
        if let Some(Ordering::Equal) = order {
            other.tweet_id.partial_cmp(&self.tweet_id)
        } else {
            order
        }
    }
}

impl Ord for Tweet {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = other.tweet_time.cmp(&self.tweet_time);
        if Ordering::Equal == order {
            other.tweet_id.cmp(&self.tweet_id)
        } else {
            order
        }
    }
}

struct Feed<'a> {
    recent_tweets: Vec<&'a Tweet>,
}

impl<'a> Feed<'a> {
    fn new() -> Self {
        Feed { recent_tweets: Vec::with_capacity(10) }
    }

    fn insert_tweets(&mut self, tweets: &'a Vec<Tweet>) {
        for tweet in tweets.iter() {
            if self.recent_tweets.len() == 10 {
                if self.recent_tweets.last().expect("illegal state").lt(&tweet) {
                    continue;
                }
                self.recent_tweets.pop();
            }
            self.recent_tweets.push(tweet);
            self.recent_tweets.sort();
        }
    }
}

impl Twitter {
    fn new() -> Self {
        Twitter {
            relation: HashMap::new(),
            tweets: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let tweet = Tweet { tweet_id, tweet_time: Instant::now() };
        self.tweets.entry(user_id).or_insert_with(|| Vec::new()).push(tweet);
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut feed = Feed::new();
        if let Some(tweets) = self.tweets.get(&user_id) {
            feed.insert_tweets(tweets);
        }

        if let Some(followers) = self.relation.get(&user_id) {
            for follower in followers.iter() {
                let tweets = self.tweets.get(follower);
                if let Some(tweets) = tweets {
                    feed.insert_tweets(tweets)
                }
            }
        }
        feed.recent_tweets.iter().map(|&tweet| tweet.tweet_id).collect()
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        self.relation.entry(follower_id).or_insert_with(|| HashSet::new()).insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        let relation = self.relation.get_mut(&follower_id);
        if let Some(relation) = relation {
            relation.remove(&followee_id);
        }
    }
}

fn main() {
    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5);
    assert_eq!(vec![5], twitter.get_news_feed(1));
    twitter.follow(1, 2);
    twitter.post_tweet(2, 6);
    assert_eq!(vec![6, 5], twitter.get_news_feed(1));
    twitter.unfollow(1, 2);
    twitter.unfollow(1, 3);
    assert_eq!(vec![5], twitter.get_news_feed(1));
    for i in 10..100 {
        twitter.post_tweet(2, i);
    }
    twitter.follow(1, 2);
    assert_eq!(vec![99, 98, 97, 96, 95, 94, 93, 92, 91, 90], twitter.get_news_feed(1));
    for i in 10..20 {
        twitter.post_tweet(1, i);
    }
    assert_eq!(vec![19, 18, 17, 16, 15, 14, 13, 12, 11, 10], twitter.get_news_feed(1));
}
