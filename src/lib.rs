use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
  query: String,
  filename: String,
  case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next(); //skipping first value
    let query = match args.next() {
      Some(val) => val,
      None => return Err("Query is not provided"),
    };
    let filename = match args.next() {
      Some(val) => val,
      None => return Err("filename is not provided"),
    };
    let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();
    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
  pub fn run(&self) -> Result<(), Box<Error>> {
    let mut file = File::open(&self.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let result = if self.case_sensitive == true {
      search(&self.query, &contents)
    } else {
      search_case_insensitive(&self.query, &contents)
    };
    println!("{:?}", result);
    Ok(())
  }
}

fn search<'a>(query: &String, file: &'a String) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in file.lines() {
    if line.to_lowercase().contains(&query.to_lowercase()) {
      result.push(line)
    }
  }
  result
}
fn search_case_insensitive<'a>(query: &String, file: &'a String) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in file.lines() {
    if line.contains(query) {
      result.push(line)
    }
  }
  result
}
