use std::thread;
use std::time::Duration;

fn main() {
  // let handle = thread::spawn(|| {
  //   for i in 1..=10 {
  //     println!("Hi number {} from the spawned thread!", i);
  //     thread::sleep(Duration::from_millis(1));
  //   }
  // });
  
  // for i in 1..=5 {
  //   println!("Hi number {} from the main thread!", i);
  //   thread::sleep(Duration::from_millis(1));
  // }
  
  // handle.join().unwrap();


  // ---------- Using move with closures ----------

  let v = vec![1, 2, 3];
  
  let handle = thread::spawn(move || {
    println!("Here's a vector {:?}", v);
  });
  
  handle.join().unwrap();
}
