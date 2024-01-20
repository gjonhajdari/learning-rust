use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
  // // ----------
  // let greeting_file_result = File::open("hello.txt");
  
  // // Code panics on any error besides the missing file error
  // let greeting_file = match greeting_file_result {
  //   Ok(file) => file,
  //   Err(error) => match error.kind() {
  //     ErrorKind::NotFound => match File::create("hello.txt") {
  //       Ok(fc) => fc,
  //       Err(e) => panic!("Problem creating file: {}", e),
  //     },
  //     other_error => panic!("Problem opening the file: {:?}", other_error),
  //   }
  // };
  // // ----------

  // // ----------
  // // Alternative writing of function using closures
  // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
  //   if error.kind() == ErrorKind::NotFound {
  //     File::create("hello.txt").unwrap_or_else(|error| {
  //       panic!("Problem creating file: {:?}", error);
  //     })
  //   } else {
  //     panic!("Problem opening file: {:?}", error);
  //   }
  // });
  // // ----------

  // // ----------
  // // Shorthand method using unwrap
  // let greeting_file = File::opne("hello.txt").unwrap();
  // // Shorthand method using expect and panic error message
  // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
  // // ----------

  // ----------
  // Propagating errors
  let username = match read_username_from_file("hello.txt") {
    Ok(username) => username,
    Err(e) => panic!("{}", e),
  };

  println!("Username: {}", username);
  
  let last_char = last_char_first_line(&username);
  
  match last_char {
    Some(c) => println!("Last character: {}", c),
    None => println!("Please provide a non-empty string."),
  };
  // ----------
}


fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
  let username_file_result = File::open(file_name);

  let mut username_file = match username_file_result {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut username = String::new();

  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e),
  }
}


// // Shorthand using the `?` operator
// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
//   let mut username_file = File::open(file_name)?;
//   let mut username = String::new();
//   username_file.read_to_string(&mut username)?;

//   Ok(username)
// }


// // Shortening even further by chaining methods
// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
//   let mut username = String::new();

//   File::open(file_name)?.read_to_string(&mut username)?;

//   Ok(username)
// }


// // Shortening even further by using `fs::read_to_string` directly
// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
//   fs::read_to_string(file_name)
// }


// The `?` operator for functions returning Option<T> types
fn last_char_first_line(text: &str) -> Option<char> {
  text.lines().next()?.chars().last()
}


// // Being able to use `?` on the main function
// use std::error::Error;
// use std::fs::File;

// fn main() -> Result<(), Box<dyn Error>> {
//   let greeting_file = File::open("hello.txt")?;

//   Ok(())
// }