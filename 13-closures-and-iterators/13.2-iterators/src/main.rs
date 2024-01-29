fn main() {
  // let v1 = vec![1, 2, 3];

  // let v1_iter = v1.iter();

  // for val in v1_iter {
  //   println!("Got {}", val);
  // }

  let v1 = vec![1, 2, 3];

  let v2: Vec<_> = v1.iter().map(|x| x * 2).collect();

  println!("v1: {:?}", v1);
  println!("v2: {:?}", v2);
}
