use std::fmt::Display;

// // Won't compile
// fn longest(x: &str, y: &str) -> &str {
//   if x.len() > y.len() {
//     x
//   } else {
//     y
//   }
// }


// Lifetime anotations in functions
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}


// Lifetime annotations in structs
struct ImportantExcerpt<'a> {
  part: &'a str,
}


// Lifetime annotations in methods
impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {
    3
  }

  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}


// The static lifetime
// let s: &'static str = "I have a static lifetime.";


fn main() {
  // &i32        // reference
  // &'a i32     // reference with an explicit lifetime
  // &'a mut i32 // mutable reference with an explicit lifetime


  // let string1 = String::from("abcd");
  // let string2 = "xyz";

  // let result = longest(string1.as_str(), string2);
  // println!("The longest string is {}", result);


  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split(".").next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
    part: first_sentence,
  };
}


// Generic type parameters, trait bounds and lifetimes together
fn longest_with_announcement<'a, T>(
  x: &'a str,
  y: &'a str,
  ann: T
) -> &'a str
where T: Display
{
  println!("Announcement: {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}