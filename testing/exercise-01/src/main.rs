fn main() {
  // println!("Hello, world!");
  let temp_c = 0;
  let temp_f = celcius_to_fahrenheit(temp_c);

  println!("Celcius: {temp_c}°C");
  println!("Fharenheit: {temp_f}°F");
}

fn celcius_to_fahrenheit(temp_c: i32) -> i32 {
  return temp_c * (9 / 5) + 32;
}

fn fharenheit_to_celcius(temp_f: i32) -> i32 {
  return 5 / 9 * (temp_f - 32);
}