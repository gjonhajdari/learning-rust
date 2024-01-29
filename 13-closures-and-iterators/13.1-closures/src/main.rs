use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
  Red,
  Blue,
}

struct Inventory {
  shirts: Vec<ShirtColor>,
}

impl Inventory {
  fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  fn most_stocked(&self) -> ShirtColor {
    let mut num_red = 0;
    let mut num_blue = 0;

    for color in &self.shirts {
      match color {
        ShirtColor::Red => num_red += 1,
        ShirtColor::Blue => num_blue += 1,
      }
    }

    if num_red > num_blue {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}


fn main() {
  // let store = Inventory {
  //   shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
  // };

  // let user_pref1 = Some(ShirtColor::Red);
  // let giveaway1 = store.giveaway(user_pref1);
  // println!(
  //   "The user with preference {:?} gets {:?}",
  //   user_pref1, giveaway1
  // );

  // let user_pref2 = None;
  // let giveaway2 = store.giveaway(user_pref2);
  // println!(
  //   "The user with preference {:?} gets {:?}",
  //   user_pref2, giveaway2
  // );


  // // Annotating closure types
  // let expensive_closure = |num: u32| -> u32 {
  //   println!("Calculating slowly.");
  //   thread::sleep(Duration::from_secs(2));

  //   num
  // };


  // // Capturing references
  // let mut list = vec![1, 2, 3];
  // println!("Before defining closure: {:?}", list);

  // // let only_borrows = || println!("From closure: {:?}", list);
  // let mut borrows_mutably = || list.push(7);

  // // println!("Before calling closure: {:?}", list);
  // // only_borrows();
  // borrows_mutably();
  // println!("After calling closure: {:?}", list);


  // // Moving references
  // let list = vec![1, 2, 3];
  // println!("Before defining closure: {:?}", list);

  // thread::spawn(move || println!("From thread: {:?}", list))
  //   .join()
  //   .unwrap();


  // // Fn traits: FnMut
  let mut list = [
    Rectangle { width: 10, height: 1 },
    Rectangle { width: 3, height: 5 },
    Rectangle { width: 7, height: 12 },
    Rectangle { width: 30, height: 20 },
  ];

  // list.sort_by_key(|rect| rect.width);
  // println!("{:#?}", list);

  // // Won't compile if we pass a closure implementing FnOnce
  // let mut sort_operations = vec![];
  // let value = String::from("by key called");

  // list.sort_by_key(|rect| {
  //   sort_operations.push(value);
  //   rect.width
  // });

  // println!("{:#?}", list);

  // Will compile because closure implements FnMut
  let mut num_sort_operations = 0;
  list.sort_by_key(|rect| {
    num_sort_operations += 1;
    rect.width
  });

  println!("{:#?}, sorted in {num_sort_operations} operations", list);
}


// // Closure vs function syntax
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;


// // Fn traits: FnOnce
// impl<T> Option<T> {
//   pub fn unwrap_or_else<F>(self, f: F) -> 
//   where
//     F: FnOnce() -> T
//   {
//     match self {
//       Some(x) => x,
//       None => f(),
//     }
//   }
// }
