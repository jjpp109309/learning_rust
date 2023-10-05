pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

pub trait DefaultSummary {
    fn default_summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// implementing traits on a types
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("(author: {})", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl DefaultSummary for Tweet {}

// define a function that inputs any type that implements a trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());    
}

fn returns_summarizable() -> impl Summary {
    // must return the same type :)
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    }
}

// conditionaly implement methods with trait bounds
use std::fmt::Display;
use std::cmp::PartialOrd;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display+PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is {}", self.x);
        } else {
            println!("The largest member is {}", self.y);
        }
    }
}


