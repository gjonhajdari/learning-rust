use std::collections::HashMap;

fn find_mode(v: &mut [i32]) -> Option<i32> {
  let mut map = HashMap::new();

  for num in v.to_vec() {
    let count = map.entry(num).or_insert(0);
    *count += 1;
  }

  let mut most_occurances = 0;
  let mut mode = None;

  for (key, value) in &map {
    if value > &most_occurances {
      most_occurances = *value;
      mode = Some(*key);
    } else if *value == most_occurances {
      mode = None;
    }
  }

  return mode;
}

fn find_median(vec: &mut [i32]) -> Option<f64> {
  vec.sort();

  let middle = vec.len() / 2;

  if vec.is_empty() {
    return None;
  }

  if vec.len() % 2 == 0 {
    return Some((vec[middle - 1] + vec[middle]) as f64 / 2.0);
  } else {
    return Some(vec[middle] as f64);
  }
}

fn find_mean(vec: &[i32]) -> Option<f64> {
  if vec.is_empty() {
    return None;
  }

  let mut sum = 0;

  for num in vec {
    sum += *num;
  }

  let mean = sum as f64 / vec.len() as f64;

  return Some(mean)
}



fn main () {
  let mut v = vec![1, 2, 3, 4, 5, 6, 5, 5, 7, 8, 2, 5, 10, 2, 7, 3, 9, 0, 8, 3, 9, 0, 6, 6, 6, 6];

  let mode = find_mode(&mut v);
  let median = find_median(&mut v);
  let mean = find_mean(&mut v);

  match mode {
    Some(mode) => println!("Mode: {}", mode),
    None => println!("The vector doesn't have a mode."),
  }

  match median {
    Some(median) => println!("Median: {}", median),
    None => println!("The vector is empty."),
  }

  match mean {
    Some(mean) => println!("Mean: {}", mean),
    None => println!("The vector is empty."),
  }
}
