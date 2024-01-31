//! # My Crate
//! 
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the given number.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(answer, 6);
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_adds_one() {
    assert_eq!(add_one(5), 6);
  }
}