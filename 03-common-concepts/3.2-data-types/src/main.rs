fn main() {
  println!("----- TYPES -----");
  // --- Basic Types ---
  // Float type
  let x = 2.0; // f64
  let y: f32 = 3.0; // f32

  // Boolean type
  let t = true;
  let f: bool = false;

  // Char type
  let c = 'z';
  let z: char = 'ℤ';
  let heart_eyed_cat = '😻';

  // --- Compound Types ---
  // Touples
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  
  println!("The value of x is: {x}");
  println!("The value of y is: {y}");
  println!("The value of z is: {z}");
  
  let first_value = tup.0;
  println!("The value of x is: {first_value}");

  // Arrays
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let b = [3; 5];
  let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

  let first = a[0];
  let second = a[1];


  println!("----- OPERATIONS -----");
  // --- Operations ---
  // addition
  let sum = 5 + 3;

  // subtraction
  let difference =  95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1

  // remainder
  let remainder = 43 % 5;

  println!("Sum: {sum}");
  println!("Difference: {difference}");
  println!("Product: {product}");
  println!("Quotient: {quotient}");
  println!("Truncated: {truncated}");
  println!("Remainder: {remainder}");
}


// ----- EXTRA ARRAY CODE -----
// use std::io;

// fn main() {
//   let a = [1, 2, 3, 4, 5];

//   println!("Please enter an array index");

//   let mut index = String::new();

//   io::stdin()
//     .read_line(&mut index)
//     .expect("Failed to read line!");

//   let index: usize = index
//     .trim()
//     .parse()
//     .expect("Index entered was not a number");

//   let element = a[index];

//   println!("The value of the element at index {index} is: {element}");
// }