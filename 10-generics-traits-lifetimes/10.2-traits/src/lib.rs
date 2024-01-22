pub trait Summary {
  // fn summarize(&self) -> String;

  // // Default behaviour for all types implementing trait
  // fn summarize(&self) -> String {
  //   String::from("(Read more...)")
  // }

  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

// Implementing with override
// impl Summary for NewsArticle {
//   fn summarize(&self) -> String {
//     format!("{}, by {} ({})", self.headline, self.author, self.location)
//   }
// }

// Implementing with default behaviour
impl Summary for NewsArticle {}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  // fn summarize(&self) -> String {
  //   format!("{}: {}", self.username, self.content)
  // }

  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}


// ---------- Traits as parameters ----------

pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
pub fn notify(item: &impl Summary) {}
pub fn notify<T: Summary>(item: &T) {}

// Allowing item1 and item2 to have different types
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

// Forcing both parameters to have the same type
pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// Multiple trait bounds using `+`
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}

// Clearer trait bounds using the `where` sytax
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
  T: Display + Clone,
  U: Clone + Debug,
{}


// ---------- Returning types that implement traits ----------

fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
}

// Wouldn't work
fn returns_summarizable(switch: bool) -> impl Summary {
  if switch {
    NewsArticle {
      headline: String::from("Penguins win the Stanley Cup Championship!",),
      location: String::from("Pittsburgh, PA, USA"),
      author: String::from("Iceburgh"),
      content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.",),
    }
  } else {
    Tweet {
      username: String::from("horse_ebooks"),
      content: String::from("of course, as you probably already know, people"),
      reply: false,
      retweet: false,
    }
  }
}


// ---------- Returning types that implement traits ----------

use std::fmt::Display;

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) {
    Self { x, y, }
  }
}

// Only implements `cmp_display` for types that implement `Display` and `PartialOrd`
impl<Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x > self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}

// Blanket implementations
impl<T: Display> ToString for T {}


// --------------------

// // Example how a binary crate could use our library crate
// use aggregator::{Summary, Tweet};

// fn main() {
//   let tweet = Tweet {
//     username: String::from("horse_ebooks"),
//     content: String::from("of course, as you probably already know, people"),
//     reply: false,
//     retweet: false,
//   };

//   println!("1 new tweet: {}", tweet.summarize());


//   let article = NewsArticle {
//     headline: String::from("Penguins win the Stanley Cup Championship!"),
//     location: String::from("Pittsburgh, PA, USA"),
//     author: String::from("Iceburgh"),
//     content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
//   };

//   println!("New article available! {}", article.summarize());
// }