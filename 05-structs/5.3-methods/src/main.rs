#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn width(&self) -> bool {
    self.width > 0
  }

  fn can_hold(&self, other: &Rectangle ) -> bool {
    self.width > other.width && self.height > other.height
  }

  // Associated functions
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  // let rect2 = Rectangle { width: 20, height: 40};
  // let rect3 = Rectangle { width: 30, height: 60};

  // println!("The area of the rectangle is {} square pixels.", rect1.area());

  // if rect1.width() {
  //   println!("The rectangle has a non-zero width. It is: {}px", rect1.width);
  // }

  // println!("Can 'rect1' hold 'rect2'? {}", rect1.can_hold(&rect2));
  // println!("Can 'rect1' hold 'rect3'? {}", rect1.can_hold(&rect3));
  // println!("Can 'rect3' hold 'rect2'? {}", rect3.can_hold(&rect2));

  let rect4 = Rectangle::square(100);
  println!("The area of the square is {} square pixels", rect4.area());
}