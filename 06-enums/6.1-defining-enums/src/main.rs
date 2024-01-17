// // Version one
// #[derive(Debug)]
// enum IpAddrKind {
//   V4(String),
//   V6(String),
// }

// Version two
#[derive(Debug)]
enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String),
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    println!("Hello!");
  }
}

fn main() {
  // // Version one
  // let four = IpAddrKind::V4(String::from("127.0.0.1"));
  // let loopback = IpAddrKind::V6(String::from("::1"));
  
  // // Version two
  // let four = IpAddrKind::V4(127, 0, 0, 1);
  // let loopback = IpAddrKind::V6(String::from("::1"));

  // println!("Four: {:?}", four);
  // println!("Loopback: {:?}", loopback);

  // let m = Message::Write(String::from("!!!"));
  // m.call();

  // ---------- Option enum
  let some_number = Some(5);
  let some_char = Some('e');

  let absent_number: Option<i32> = None;

}