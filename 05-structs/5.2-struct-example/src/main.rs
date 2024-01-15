#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  // // Example one (two ingeger values)
  // let width1 = 30;
  // let height1 = 50;

  // println!(
  //   "The area of a rectangle is {} square pixels.",
  //   area(width1, height1)
  // );

  // // Example two (touples)
  // let rect1 = (30, 50);

  // println!(
  //   "The area of a rectangle is {} square pixels.",
  //   area(rect1)
  // );

  // // Example three (structs)
  // let rect1 = Rectangle {
  //   width: 30,
  //   height: 50,
  // };

  // println!(
  //   "The area of a rectangle is {} square pixels.",
  //   area(&rect1)
  // );

  // println!("rect1 is: '{:#?}'", rect1);

  // Printing with debug
  let scale = 2;
  let rect2 = Rectangle {
    width: dbg!(30 * scale),
    height: 50,
  };

  dbg!(rect2);
}

// // Example one (two ingeger values)
// fn area(width: u32, height: u32) -> u32 {
//   width * height
// }

// // Example two (touples)
// fn area(dimensions: (u32, u32)) -> u32 {
//   dimensions.0 * dimensions.1
// }

// Example three (structs)
// fn area(rectangle: &Rectangle) -> u32 {
//   rectangle.width * rectangle.height
// }