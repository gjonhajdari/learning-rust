use blog::Post;

fn main() {
  // let mut post = Post::new();
  // post.add_text("I ate salad for lunch today");
  // post.request_review();
  // post.approve();
  // assert_eq!(post.content(), "I ate salad for lunch today");

  let mut post = Post::new();
  post.add_text("I ate salad for lunch today");

  let post = post.request_review();
  
  let post = post.approve();
  
  println!("Content: {}", post.content());
}