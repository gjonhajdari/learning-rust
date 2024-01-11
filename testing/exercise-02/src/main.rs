fn main() {
  let n = 30;
  println!("fib({n}) = {}", fib(n));
}

fn fib(n: i128) -> i128 {
  if n <= 0 {
    return 0;
  } else if n == 1 {
    return 1;
  } else {
    return fib(n - 1) + fib(n - 2);
  }
}