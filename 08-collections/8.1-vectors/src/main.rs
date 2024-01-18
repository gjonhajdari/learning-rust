fn main() {
  // // ----------
  // let v: Vec<i32> = Vec::new();
  // // ----------
  
  // // ----------
  // let v = vec![1, 2, 3];
  // println!("Vector -> {:?}", v);
  // // ----------

  // // ----------
  // let mut v = Vec::new();

  // v.push(5);
  // v.push(6);
  // v.push(7);
  // v.push(8);
  // // ----------

  // // ----------
  // let v = vec![1, 2, 3, 4, 5];

  // let third: &i32 = &v[2];
  // println!("The third element is: {}", third);

  // let thord: Option<&i32> = v.get(2);
  // match thord {
  //   Some(thord) => println!("The thord element is: {}", thord),
  //   None => println!("There is no thord element."),
  // }
  // // ----------

  // // ----------
  // let v = vec![1, 2, 3, 4, 5];

  // let does_not_exist = &v[100];
  // let does_exist = v.get(100);
  // // ----------

  // ----------
  // let v = vec![100, 32, 57];
  // for i in &v {
  //   println!("{i}");
  // }

  // let mut v = vec![100, 32, 57];
  // for i in &mut v {
  //   *i += 50;
  // }
  // ----------

  // ----------
  #[derive(Debug)]
  enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }
  
  let row = vec![
    SpreadSheetCell::Int(3),
    SpreadSheetCell::Text(String::from("blue")),
    SpreadSheetCell::Float(10.12),
  ];
  // ----------

  println!("Row -> {:?}", row);
}

