fn main() {
  // panic!("crash and burn");

  let v = vec![1, 2, 3];

  // Run with `RUST_BACKTRACE=1 cargo run`
  v[99];
}
