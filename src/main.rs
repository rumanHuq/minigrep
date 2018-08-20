/**
 * 1. get arguments from cli and check if it is available in a text file
 */

extern crate minigrep;
use minigrep::Config;
use std::process;

fn main(){
  let config = Config::new(std::env::args()).unwrap_or_else(|err_message| {
    eprintln!("{}", err_message);
    process::exit(1);
  });

  if let Err(val) = config.run() {
    eprintln!("{}", val);
    process::exit(1);
  }
}
