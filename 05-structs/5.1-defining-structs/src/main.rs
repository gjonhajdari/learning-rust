// ---------- STRUCTS ----------
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

// ---------- TOUPLE STRUCTS ----------
struct Color(i8, i8, i8);
struct Point(i32, i32, i32);

// ---------- UNIT-LIKE STRUCTS ----------
struct AlwaysEqual;

fn main() {
  // // ----------
  // let mut user1 = User {
  //   active: true,
  //   username: String::from("chonatan"),
  //   email: String::from("gjonhajdari1@gmail.com"),
  //   sign_in_count: 1,
  // };

  // user1.email = String::from("gjon.hajdari@student.uni-pr.edu");
  // // ----------

  // // ----------
  // let username = String::from("chonatan");
  // let email = String::from("gjonhajdari1@gmail.com");

  // let user1 = build_user(username, email);
  // // ----------

  // // ----------
  // // Not using struct update syntax
  // let user2 = User {
  //   active: user1.active,
  //   username: user1.username,
  //   email: String::from("gjon.hajdari@student.uni-pr.edu"),
  //   sign_in_count: user1.sign_in_count,
  // };
  
  // // Using struct update syntax
  // let user2 = User {
  //   email: String::from("gjon.hajdari@student.uni-pr.edu"),
  //   ..user1
  // };
  // // ----------

  // // ----------
  // let black = Color(0, 0, 0);
  // let origin = Point(0, 0, 0);
  // // ----------
  
  // ----------
  let subject = AlwaysEqual;
  // ----------

  // println!("{} - {}", user1.username, user1.email);
}

fn build_user(username: String, email: String) -> User {
  User {
    username,
    email,
    active: true,
    sign_in_count: 1
  }
}