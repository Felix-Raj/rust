#[allow(unused_variables)]

use core::fmt::Debug;

trait Summary {
    // let user define the implementation
    fn summarize_author(&self) -> String ;
    // or can also provide a default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[derive(Debug)]
struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // use default implementation of summarize
    // but summarize_author need to be implemented
    fn summarize_author(&self) -> String {
        "news_article_aggregator".to_string()
    }
}

#[derive(Debug)]
struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {

    fn summarize_author(&self) -> String {
        // cannnot directly return self.username
        //      self.username is of String type, 
        //      trying to return self.username will
        //      try to change ownership - try to *move*
        //      see - https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope
        // so either 
        // format!("@{}", self.username)
        // or
        self.username.clone()  // clone creates a copy
        // or 
        // let uname = self.username;
        // uname.clone()
    }

    fn summarize(&self) -> String {
        // cannot call default implementation from overriding implementation
        format!("{}: @{} ", self.content, self.username)
    }
}

fn notify(item: &impl Summary) {
    // the above function signature is a shorthand see notify2
    // can be called with any item implementing trait Summary
    println!("Important... {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    /*
    This form in suitable in certain cases like
        pub fn notify(item1: impl Summary, item2: impl Summary) {
    can be written better as
        pub fn notify<T: Summary>(item1: T, item2: T) {
    */
    println!("Important... {}", item.summarize());
}

/*
To specify more than one trait, use +
    notify(item: impl Summerize + Display)
which is equivalent to 
    notify<T: Summerize + Display>(item: T)
*/

fn some_function<T: Summary + Clone, U: Clone + Debug>(t: T, u: U) -> i32 { 90 }
// can be written using where clause
fn some_function2<T, U>(t: T, u: U) -> i32 
    where T: Summary + Clone, 
          U: Clone + Debug {
    // function body starts after where clause
    90
}

fn main() {
    /*
    Trait is similar to Interfaces, with some differences.
    Types implementing the trait should provide the implementation body for the trait.
    Either the trait or the type should be local to our crate - orphan rule.
    */
    let tweet1 = Tweet {
        username: "User Name".to_string(),
        content: "Content".to_string(),
        reply: false,
        retweet: true,
    };
    notify(&tweet1);

    let newsarticle = NewsArticle {
        headline: "Headline".to_string(),
        content: "Content".to_string(),
        author: "Author".to_string(),
        location: "Location".to_string(),
    };
    notify2(&newsarticle)
}