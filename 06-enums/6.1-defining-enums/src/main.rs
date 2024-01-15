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

fn main() {
  // let four = IpAddrKind::V4(String::from("127.0.0.1"));
  // let loopback = IpAddrKind::V6(String::from("::1"));
  let four = IpAddrKind::V4(127, 0, 0, 1);
  let loopback = IpAddrKind::V6(String::from("::1"));

  println!("Four: {:?}", four);
  println!("Loopback: {:?}", loopback);
}
