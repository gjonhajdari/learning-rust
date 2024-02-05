use gui::{Draw, Button, Screen};

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    // code for drawing the select button
  }
}

fn main() {
  // // String doesn't implement `Draw`
  // let screen = Screen {
  //   components: vec![Box::new(String::from("Hi!"))],
  // };

  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
      }),
    ],
  };

  screen.run();
}