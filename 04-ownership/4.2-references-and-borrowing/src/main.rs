fn main() {
  let mut s1 = String::from("Hello");
  
  // // ----------
  // // immutable borrow
  // let r1 = &s1;
  // let len = calculate_length(&r1);
  
  // println!("The length of '{}' is {}", r1, len);

  // // mutable borrow
  // let r2 = &mut s1;
  // change(&mut s1);
  // let len = calculate_length(&s1);
  
  // println!("The length of '{}' is {}", s1, len);
  // // ----------

  // // ----------
  // // reference scopes
  // {
  //   let r1 = &mut s1;
  //   println!("{}", r1);
  // }

  // let r1 = &s1; // no problem
  // let r2 = &s1; // no problem
  // println!("{} and {}", r1, r2);
  // // variables aren't being used after this so their scope ends

  // let r3 = &mut s1; // no problem
  // println!("{}", r3);
  // // ----------

  let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
  s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
  some_string.push_str(" World!");
}

fn dangle() -> &String {  // dangle returns a reference to a String
  let s = String::from("Hello");  // s is a new String

  &s  // we return a reference to the String, s
}     // Here, s goes out of scope, and is dropped. Its memory goes away.
      // Danger!