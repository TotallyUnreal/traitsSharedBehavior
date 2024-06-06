use core::fmt::Display;
use std::fmt::Debug;
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("User"),
        content: String::from("of course, as you already know."),
        reply: false,
        retweet: false,
    };
    println!("1 new X post: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Man finds cure for all!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("Turns out the man is a fruad!"),
    };

    println!("New article available! {}", article.summarize());
}

// Only allows datatypes that implement the trait of Summary
// This version is known as Traits as Parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// This is Trait bound syntax. This is basically the same as the above function
// but with more syntax suger
// This method is good if you want to force two or more parameters to be the same type
// The Traits as Parameters will allow different datatypes as long as they both implement the same
// trait
pub fn notify_2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Say you what to only allow a datatype that implements two or more traits that you specify
// there are two ways to do this
pub fn notify_3<T: Summary + Display>(item: &T) {
    println!("Breaking news! {:?}", item.summarize());
}
// and
pub fn notify_4(item: &(impl Summary + Display)) {
    println!("Breaking news! {:?}", item.summarize());
}
// Please note use the one with generics if two or more parameters should take the same datatype

// Using too many trait bounds can make your code cluttered even moreso if each generic has its own
// trait bounds. To remedy this the where clause allows us to define it in a clearer manner
// BEFORE:
fn foo<T: Display + Debug, U: Summary + Display>(first: &T, sec: &U) -> i32 {
    println!("Bla bla bla");
    267
}
// AFTER:
fn foo2<T, U>(first: &T, sec: &U) -> i32
where
    T: Display + Debug,
    U: Display + Clone,
{
    println!("Blua blaub aalue");
    123
}

// This is how to return a datatype that implements a trait of Clone
// This is especially useful in the context of closures and iterators
// You can only use impl Trait for a return value if that is the only type you are returning.
// It is possable to return dfferent types that implement a specific type but not with our current
// tools in our tool box
fn cloneable() -> impl Clone {
    String::new()
}

// Using Trait bounds to conditionally implement methods
// This means that we can implement a specific method for a String, i32 or bool
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("The larger of the pair is: {}", self.first);
        } else {
            println!("The larger of the pair is: {}", self.second);
        }
    }
}
