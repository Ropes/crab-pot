#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("author: {}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// accept a trait as function parameter.
// similar to accepting an interface in Go.
pub fn notify(item: impl Summary) {
    println!("breaking news! {}", item.summarize())
}

// trait bound syntax for accepting generic type
// more verbose
pub fn notify_summary<T: Summary>(item: T) {
    println!("breaking news: {}", item.summarize())
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
/* see page 375 for fix
fn returns_summarizable(switch: bool) -> impl Summary{
    if switch{
        NewsArticle{
            headline: String::from("Penguins win stanley cup"),
            location: String::from("Pittsburgh, PA"),
            author: String::from("Iceburgh"),
            content: String::from("Pengiuins win again!"),
        }
    }else{
        Tweet{
            username: String::from("horse_eboosk"),
            content: String::from("why the long face?"),
            reply: false,
            retweet: false,
        }
    }
}
*/

// longest string returned using parameter lifetime annotation
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
// lifetime of struct is asociated through 'self'
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;
