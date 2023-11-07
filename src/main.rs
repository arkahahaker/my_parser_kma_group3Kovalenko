use pls_parser::parse;
use std::fs;
use std::path::Path;
use clap::{App, Arg};
use peg::error::ParseError;
use peg::str::LineCol;

pub fn main() -> Result<(), ParseError<LineCol>> {
    let matches = App::new("PLS parser")
        .author("arkananasfa")
        .version("0.1.0")
        .about("Parse PLS files to console")
        .arg(Arg::with_name("file")
            .help("Print the path to the file: ")
            .required(true)
            .index(1))
        .get_matches();
  
    let path = Path::new(matches.value_of("file").unwrap());
  
    if path.is_file() {
      match fs::read_to_string(&path) {
        Ok(contents) => {
          parse(&contents);
        }
        Err(err) => {
          eprintln!("Can't reading file. {}", err);
        }
      }
    } else {
      eprintln!("File not exist.");
    }
  
    Ok(())
}