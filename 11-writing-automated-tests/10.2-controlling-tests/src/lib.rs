// fn prints_and_returns_10(a: i32) -> i32 {
//   println!("I got the value {}", a);
//   10
// }

pub fn add_two(a: i32) -> i32 {
  a + 2
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn this_test_will_pass() {
  //   let value = prints_and_returns_10(4);
  //   assert_eq!(value, 10);
  // }

  // #[test]
  // fn this_test_will_fail() {
  //   let value = prints_and_returns_10(8);
  //   assert_eq!(value, 5);
  // }

  #[test]
  fn add_two_and_two() {
    assert_eq!(add_two(2), 4);
  }

  #[test]
  fn add_three_and_two() {
    assert_eq!(add_two(3), 5);
  }

  #[test]
  fn one_hundred() {
    assert_eq!(add_two(100), 102);
  }

  #[test]
  #[ignore]
  fn expensive_test() {
    // code that takes an hour to run
  }
}