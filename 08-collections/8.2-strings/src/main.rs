fn main() {
  // let mut s = String::new();

  // // ----------
  // let data = "initial contents";
  // let s = data.to_string();

  // Works on literals directly
  // let s = "initial contents".to_string();
  // // ----------

  // // ----------
  // let s = String::from("initial contents");

  // let hello = String::from("السلام عليكم");
  // let hello = String::from("Dobrý den");
  // let hello = String::from("Hello");
  // let hello = String::from("שָׁלוֹם");
  // let hello = String::from("नमस्ते");
  // let hello = String::from("こんにちは");
  // let hello = String::from("안녕하세요");
  // let hello = String::from("你好");
  // let hello = String::from("Olá");
  // let hello = String::from("Здравствуйте");
  // let hello = String::from("Hola");
  // // ----------

  
  // ---------- Updating
  // let mut s = String::from("foo");
  // s.push_str("bar"); // result -> foobar
  
  // let mut s1 = String::from("foo");
  // let s2 = "bar";
  // s1.push_str(s2);
  // println!("s1 = {}, s2 = {}", s1, s2);
  
  // let mut s = String::from("lo");
  // s.push('l');

  // let s1 = String::from("Hello ");
  // let s2 = String::from("World");
  // let s3 = s1 + &s2; // '+' -> fn add(self, s: &str) -> String {...}

  // let s1 = String::from("tic");
  // let s2 = String::from("tac");
  // let s3 = String::from("toe");

  // let s = s1 + "-" + &s2 + "-" + &s3;
  // let s = format!("{s1}-{s2}-{s3}"); // Easier to read
  // println!("{s}");
  // ----------

  // ----------
  println!("Chars:");

  for c in "Зд".chars() {
    println!("{c}");
  }

  println!("Bytes:");

  for b in "Зд".bytes() {
    println!("{b}");
  }
  // ----------
}
