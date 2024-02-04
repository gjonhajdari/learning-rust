#[derive(Debug, PartialEq)]
pub struct AverageCollection {
  list: Vec<i32>,
  average: f64,
}

impl AverageCollection {
  pub fn add(&mut self, value: i32) {
    self.list.push(value);
    self.update_average();
  }

  pub fn remove(&mut self) -> Option<i32> {
    let result = self.list.pop();

    match result {
      Some(value) => {
        self.update_average();
        Some(value)
      }
      None => None,
    }
  }

  pub fn average(&self) -> f64 {
    self.average
  }

  fn update_average(&mut self) {
    let total: i32 = self.list.iter().sum();
    self.average = total as f64 / self.list.len() as f64;
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_adds_to_list() {
    let mut ac = AverageCollection {
      list: vec![],
      average: 0.0,
    };

    ac.add(1);
    ac.add(2);
    ac.add(3);

    assert_eq!(ac.list, vec![1, 2, 3]);
  }

  #[test]
  fn it_removes_from_list() {
    let mut ac = AverageCollection {
      list: vec![1, 2, 3],
      average: 2.0,
    };

    ac.remove();

    assert_eq!(ac.list, vec![1, 2]);
  }

  #[test]
  fn it_updates_average() {
    let mut ac = AverageCollection {
      list: vec![],
      average: 0.0,
    };

    ac.add(1);
    ac.add(2);
    ac.add(3);
    ac.add(4);

    ac.remove();

    assert_eq!(ac.average(), 2.0);
  }
}