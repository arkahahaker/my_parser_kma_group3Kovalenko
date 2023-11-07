extern crate peg;

use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Song {
    pub file: String,
    pub title: String,
    pub length: Option<u32>,
}

peg::parser! {
    pub grammar playlist_parser() for str {
        pub rule playlist() -> Vec<Song>
            = "[playlist]" eol() "Version=" number() eol() "NumberOfEntries=" number() eol()
            entry()* { Vec::new() }

        pub rule entry() -> Song
            = "File" number() "=" filename:filename() eol()
              "Title" number() "=" title:title() eol()
              "Length" number() "=" length:number()? eol()
              { Song { file: filename.to_string(), title: title.to_string(), length } }

        pub rule filename() -> String
            = filename:$(['A'..='Z' | 'a'..='z' | '0'..='9' | '/' | '.' | ' ' | '-' | '(' | ')' | '&' | '!' | ',' | '+' | '\'' | '#']+) { filename.to_string() }

        pub rule title() -> String
            = title:$(['A'..='Z' | 'a'..='z' | '0'..='9' | ' ' | '-' | '/' | '.' | '(' | ')' | '&' | '!' | ',' | '+' | '\'' | '#' | ']' | '[' | ']' | '[' | ':' | ';' | '?' | '$' | '%' | '^' | '*' | '@' | '=' | '~' | '`']+) { title.to_string() }

        pub rule number() -> u32
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule eol()
            = "\r"? "\n"
    }
}

pub fn parse(path: &str) -> &str {
  let contents = fs::read_to_string(path).expect("Failed to read file");

    let parsed = match playlist_parser::playlist(&contents) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error parsing PLS file: {:?}", err);
            std::process::exit(1);
        }
    };

    for (index, song) in parsed.iter().enumerate() {
        match &song.length {
            Some(length) => {
                println!("Entry {}: Title: {}, File: {}, Length: {} seconds", index + 1, song.title, song.file, length);
            }
            None => {
                println!("Entry {}: Title: {}, File: {}", index + 1, song.title, song.file);
            }
        }
    }
    ""
}