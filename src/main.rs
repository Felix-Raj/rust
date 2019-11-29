#[allow(unused_variables)]

use core::fmt::Debug;
use std::fmt::Display;

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
        // format!("{}", self.username)
        // or
        self.username.clone()  // clone creates a copy
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

// Return type that implements a trait
fn returns_summarize() -> impl Summary {
    Tweet {
        username: "Another User Name".to_string(),
        content: "Yet another content".to_string(),
        reply: false,
        retweet: false,
    }
}
// but the function should have the possibility to return only one type,
// this is due to some restrictions around the implementation of `impl Trait`
// so the following will not compile, try returning NewsArticle from else arm
fn returns_summarize_2(switch: bool) -> impl Summary {
    if switch {
        returns_summarize()
    } else {
        returns_summarize()
        /*NewsArticle {
            headline: "Headline 2".to_string(),
            location: "Location Unknown".to_string(),
            author: "Anonymous".to_string(),
            content: "Classified".to_string()
        }*/
    }
}

// change uncommented to commented and vice-versa in steps
//fn largest<T>(list: &[T]) -> T {
//fn largest<T: PartialOrd>(list: &[T]) -> T {
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter(){
        if item > largest {
            largest = item;
        }
    }

    largest
}
/*
If we don't want to limit the function to types that implements Copy trait,
we could specify that T has the trait bound Clone instead of Copy. Then we can
make Clone of each value in the slice in the function. Bus with large data in heap
this will make pgm slow.
Another way is to implement to return an reference to the largest
*/
fn largest_2<T: PartialOrd>(list: &[T]) -> &T {
    // TODO: need to check dereferencing and referencing once more for this
    let mut largest = &list[0];
    for item in list.iter() {
        if *item > *largest {
            largest = item
        }
    }
    largest
}

// Using trait bounds to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// implementation of a trait can also follow similar rule
// impl<T: Display> ToString for T { }

// Traits and trait bounds let us write code that uses generic type parameters
// to reduce duplication but also specify to the compiler that we want the
// generic type to have particular behavior. Compiler can check with this
// information whether the concrete types will behave as expected.
// Checks that would have otherwise done during run time is moved to compile time

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
    notify2(&newsarticle);

    let num_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&num_list));
    println!("The largest number is {}", *largest_2(&num_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&char_list));
    println!("The largest char is {}", *largest_2(&char_list));
}