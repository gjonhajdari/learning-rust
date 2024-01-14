fn main() {
  // branching();
  repetition();
}

fn repetition() {
  // ---------- 'loop' statement ----------
  // // -----
  // loop {
  //   println!("Again!");
  // }
  // // -----

  // // -----
  // let mut counter = 0;

  // let result = loop {
  //   counter += 1;

  //   if counter == 10 {
  //     break counter * 2;
  //   }
  // };

  // println!("The result is: {result}");
  // // -----

  // // -----
  // let mut count = 0;

  // 'counting_up: loop {
  //   println!("count: {count}");
  //   let mut remaining = 10;

  //   loop {
  //     println!("remaining: {remaining}");
  //     if remaining == 5 {
  //       break;
  //     }
  //     if count == 2 {
  //       break 'counting_up;
  //     }
  //     remaining -= 1;
  //   };

  //   count += 1;
  // };

  // println!("End count: {count}");
  // // -----

  // ---------- 'while' loops ----------
  // // -----
  // let mut number = 3;

  // while number != 0 {
  //   println!("{number}!");

  //   number -= 1;
  // }

  // println!("LIFTOFF!!!");
  // // -----

  // // -----
  // let b = [10, 20, 30, 40, 50];
  // let mut index = 0;

  // while index < 5 {
  //   println!("The value of b[{index}] is: {}", b[index]);
    
  //   index += 1;
  // }
  // // -----

  // ---------- 'for' loops ----------
  // // -----
  // let a = [10, 20, 30, 40, 50];

  // for element in a {
  //   println!("The value is: {element}");
  // }
  // // -----

  // -----
  for number in (1..4).rev() {
    println!("{number}!");
  }

  println!("LIFTOFF!!!");
  // -----
}

// fn branching() {
//   // let number = 3;
  
//   // if number < 5 {
//   //   println!("The number {number} is less than 5");
//   // } else {
//   //   println!("The number {number} is greater than 5");
//   // }

//   // let number = 74126716;

//   // if number % 4 == 0 {
//   //   println!("{number} is divisible by 4");
//   // } else if number % 3 == 0 {
//   //   println!("{number} is divisible by 3");
//   // } else if number % 2 == 0 {
//   //   println!("{number} is divisible by 2");
//   // } else {
//   //   println!("{number} is not divisible by 2, 3 or 4");
//   // }

//   let condition = true;
//   let number = if condition { 5 } else { 6 };

//   println!("The value of number is: {number}")
// }