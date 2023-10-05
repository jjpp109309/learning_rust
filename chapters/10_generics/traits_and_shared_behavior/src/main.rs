use aggregator::{Tweet, Summary, DefaultSummary};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know..."),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("using default impl: {}", tweet.default_summarize());
}
