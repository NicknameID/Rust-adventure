use std::fmt::{Debug, Display};

// trait 部分
pub trait Summary {
    fn summarize(&self) -> String {
        "Read more...".to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: impl Summary) {
    println!("Breaking news {}", item.summarize())
}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
//     todo!()
// }

// multiple trait bound better way

fn some_fn<T, U>(t: T, u: U) -> i32 where T: Display + Clone,
                                          U: Clone + Debug {
    todo!()
}

fn return_summarizable_tweet() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// fn return_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best
//             hockey team in the NHL."),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }


fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
        hockey team in the NHL."),
    };
    let tweet = return_summarizable_tweet();
    println!("1 new tweet: {}", tweet.summarize());
    notify(article);

    // 使用 trait bound 有条件地实现方法
    struct Pair<T> {
        x: T,
        y: T
    }

    impl <T> Pair<T> {
        fn new(x: T, y: T) -> Self<T> {
            Self {
                x,
                y,
            }
        }
    }

    impl <T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}
