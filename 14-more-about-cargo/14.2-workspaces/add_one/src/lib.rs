use rand;

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
