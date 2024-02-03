pub trait Messenger {
  fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger
{
  pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = (self.value as f64 / self.max as f64) * 100.00;

    if percentage_of_max >= 100.00  {
      self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max >= 90.00 {
      self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentage_of_max >= 75.00 {
      self.messenger.send("Warning: You've used up over 75% of your quota!");
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn it_sends_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }
}