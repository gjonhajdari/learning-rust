// // throws error unless added to dependency list
// use rand;
use add_one;

fn main() {
  let num = 30;

  println!("Hello, world! {num} + 1 = {}", add_one::add_one(num));
}
