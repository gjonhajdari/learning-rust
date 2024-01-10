fn main() {
  // println!("Hello, world!");
  // another_function();

  // another_function(5);

  // print_labeled_measurement(5, 'L');
  
  // let x = five();
  // println!("Value returned from calling five(): {x}");

  let x = plus_one(5);
  println!("Value returned from calling plus_one(5): {x}");
}

// fn another_function(x: i32) {
//   // println!("Another function!");
//   println!("The value of x is: {x}");
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//   println!("The measurement is: {value}{unit_label}");
// }

// fn five() -> i32 {
//   5
// }

fn plus_one(x: i32) -> i32 {
  x + 1
}