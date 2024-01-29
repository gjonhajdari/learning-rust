use std::fs;
// use std::env;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
  pub query: String,
  pub file_path: String,
  #[structopt(long = "ignore-case")]
  pub ignore_case: bool,
}

impl Config {
  pub fn build<T>(mut args: T) -> Result<Config, &'static str>
  where
    T: Iterator<Item = String>
  {
    // Consumes first argument (name of the program)
    args.next();
    
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let file_path = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file path"),
    };
    
    let ignore_case = args.any(|arg| arg == "--ignore-case");
    // let ignore_case = env::var("IGNORE_CASE").is_ok(); // using environment variables
    
    Ok(Config {
      query,
      file_path,
      ignore_case,
    })
  }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &content)
  } else {
    search(&config.query, &content)
  };
  
  for line in results {
    println!("{line}");
  }

  Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}


pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let query = query.to_lowercase();

  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      search_case_insensitive(query, contents),
      vec!["Rust:", "Trust me."]
    );
  }
}