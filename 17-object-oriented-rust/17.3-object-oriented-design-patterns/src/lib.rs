// pub struct Post {
//   state: Option<Box<dyn State>>,
//   content: String,
// }

// impl Post {
//   pub fn new() -> Post {
//     Post {
//       state: Some(Box::new(Draft {})),
//       content: String::new(),
//     }
//   }

//   pub fn add_text(&mut self, text: &str) {
//     self.content.push_str(text);
//   }

//   pub fn content(&self) -> &str {
//     self.state.as_ref().unwrap().content(self)
//   }

//   pub fn request_review(&mut self) {
//     if let Some(s) = self.state.take() {
//       self.state = Some(s.request_review());
//     }
//   }

//   pub fn approve(&mut self) {
//     if let Some(s) = self.state.take() {
//       self.state = Some(s.approve());
//     }
//   }

//   pub fn reject(&mut self) {
//     if let Some(s) = self.state.take() {
//       self.state = Some(s.reject());
//     }
//   }

//   // fn type_name(&self) -> String {
//   //   self.state.as_ref().unwrap().type_name()
//   // }
// }

// trait State {
//   // fn type_name(&self) -> String;
//   fn request_review(self: Box<Self>) -> Box<dyn State>;
//   fn approve(self: Box<Self>) -> Box<dyn State>;
//   fn reject(self: Box<Self>) -> Box<dyn State>; 
//   fn content<'a>(&self, post: &'a Post) -> &'a str {
//     ""
//   }
// }

// struct Draft {}

// impl State for Draft {
//   // fn type_name(&self) -> String {
//   //   "Draft".to_string()
//   // }
  
//   fn request_review(self: Box<Self>) -> Box<dyn State> {
//     Box::new(PendingReview {})
//   }

//   fn approve(self: Box<Self>) -> Box<dyn State> {
//     self
//   }

//   fn reject(self: Box<Self>) -> Box<dyn State> {
//     self
//   }
// }

// struct PendingReview {}

// impl State for PendingReview {
//   // fn type_name(&self) -> String {
//   //   "PendingReview".to_string()
//   // }

//   fn request_review(self: Box<Self>) -> Box<dyn State> {
//     self
//   }

//   fn approve(self: Box<Self>) -> Box<dyn State> {
//     Box::new(Published {})
//   }

//   fn reject(self: Box<Self>) -> Box<dyn State> {
//     Box::new(Draft {})
//   }
// }

// struct Published {}

// impl State for Published {
//   // fn type_name(&self) -> String {
//   //   "Published".to_string()
//   // }

//   fn request_review(self: Box<Self>) -> Box<dyn State> {
//     self
//   }

//   fn approve(self: Box<Self>) -> Box<dyn State> {
//     self
//   }

//   fn reject(self: Box<Self>) -> Box<dyn State> {
//     self
//   }

//   fn content<'a>(&self, post: &'a Post) -> &'a str {
//     &post.content
//   }
// }


// ----- Implementation 2
pub struct Post {
  content: String,
}

pub struct DraftPost {
  content: String,
}

impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }

  fn type_name(&self) -> &str {
    "Post"
  }
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content,
    }
  }

  fn type_name(&self) -> &str {
    "DraftPost"
  }
}

pub struct PendingReviewPost {
  content: String,
}

impl PendingReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }

  pub fn reject(self) -> DraftPost {
    DraftPost {
      content: self.content
    }
  }

  fn type_name(&self) -> &str {
    "PendingReviewPost"
  }
}

// ----- Unit tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn approve_post() {
    let mut post = Post::new();
    post.add_text("Hello World!");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!(post.type_name(), "Post");
    assert_eq!(post.content(), "Hello World!");
  }

  #[test]
  fn reject_post() {
    let mut post = Post::new();
    post.add_text("Hello World!");

    let post = post.request_review();
    let post = post.reject();

    assert_eq!(post.type_name(), "DraftPost");
  }
}
