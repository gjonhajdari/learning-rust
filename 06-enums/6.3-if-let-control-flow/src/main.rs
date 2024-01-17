#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
  // ...
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) {
  // // Using match
  // let mut count = 0;
  // match coin {
  //   Coin::Quarter(state) => println!("State quarter from: {:?}", state),
  //   _ => count += 1,
  // }

  // Using if let with else block
  let mut count = 0;
  if let Coin::Quarter(state) = coin {
    println!("State quarter from: {:?}", state);
  } else {
    count += 1;
  }
}

fn main() {
  let config_max = Some(4u8);
  
  if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
  } 
}
