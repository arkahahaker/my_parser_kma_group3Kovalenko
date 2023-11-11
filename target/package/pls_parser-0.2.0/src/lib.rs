extern crate peg;

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
            entries:entry()* { entries }

        pub rule entry() -> Song
            = "File" number() "=" filename:filename() eol()
              "Title" number() "=" title:title() eol()
              "Length"? number()? "="? length:number()? eol()*
              { Song { file: filename, title: title, length: length } }

        pub rule filename() -> String
            = filename:$(['A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '/' | '.' | ' ' | '-' | '(' | ')' | '&' | '!' | ',' | '+' | '\'' | '#']+) { filename.to_string() }

        pub rule title() -> String
            = title:$(['A'..='Z' | 'a'..='z' | '0'..='9' | ' ' | '_' |'-' | '/' | '.' | '(' | ')' | '&' | '!' | ',' | '+' | '\'' | '#' | ']' | '[' | ']' | '[' | ':' | ';' | '?' | '$' | '%' | '^' | '*' | '@' | '=' | '~' | '`']+) { title.to_string() }

        pub rule number() -> u32
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule eol()
            = "\r"? "\n"
    }
}

pub fn parse(content: &str) -> &str {
  
    let parsed = match playlist_parser::playlist(&content) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error parsing file: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("Parsing done successfully!");
    println!("Number of songs: {}", parsed.len());
    
    println!("==================================================");
    for (index, song) in parsed.iter().enumerate() {
        match &song.length {
            Some(length) => {
                if length < &60 {
                    println!("{}: Title: {}, \r\n   File: {}, \r\n   Length: {} seconds", index + 1, song.title, song.file, length);
                } else if length % &60 == 0 {
                    let len_str = (length/60).to_string() + " minutes ";
                    println!("{}: Title: {}, \r\n   File: {}, \r\n   Length: {}", index + 1, song.title, song.file, len_str);
                } else {
                    let len_str = (length/60).to_string() + " minutes " + &((length%60).to_string()) + " seconds";
                    println!("{}: Title: {}, \r\n   File: {}, \r\n   Length: {}", index + 1, song.title, song.file, len_str);
                }
            }
            None => {
                println!("{}: Title: {}, \r\n   File: {}", index + 1, song.title, song.file);
            }
        }
        println!("==================================================");
    }
    ""
}