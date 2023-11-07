use core::num;

use pls_parser::*;
use pls_parser::Song;

#[test]
    fn test_filename_rule() {
        let filename = playlist_parser::filename("my_file.mp3").unwrap();
        assert_eq!(filename, "my_file.mp3");
    }

    #[test]
    fn test_title_rule() {
        let title = playlist_parser::title("My Song - Artist").unwrap();
        assert_eq!(title, "My Song - Artist");
    }

    #[test]
    fn test_number_rule() {
        let number = playlist_parser::number("42").unwrap();
        assert_eq!(number, 42);
        let number = playlist_parser::number("da");
        assert!(number.is_err());
    }

    #[test]
    fn test_eol_rule() {
        let eol1 = playlist_parser::eol("\n").unwrap();
        assert_eq!(eol1, ());
        let eol2 = playlist_parser::eol("\r\n").unwrap();
        assert_eq!(eol2, ());
    }

    #[test]
    fn test_entry_rule() {
        let entry_str = "File 1=my_file.mp3\nTitle 1=My Song - Artist\nLength 1=180\n";
        let entry = playlist_parser::entry(entry_str).unwrap();
        assert_eq!(
            entry,
            Song {
                file: "my_file.mp3".to_string(),
                title: "My Song - Artist".to_string(),
                length: Some(180),
            }
        );
    }

    #[test]
    fn test_playlist_rule() {
        let playlist_str = "[playlist]\nVersion=2\nNumberOfEntries=1\nFile 1=my_file.mp3\nTitle 1=My Song - Artist\nLength 1=180\n";
        let playlist = playlist_parser::playlist(playlist_str).unwrap();
        assert_eq!(
            playlist,
            vec![Song {
                file: "my_file.mp3".to_string(),
                title: "My Song - Artist".to_string(),
                length: Some(180),
            }]
        );
    }

    #[test]
    fn test_parse_valid_playlist() {
        let test_data = r#"[playlist]
Version=2
NumberOfEntries=3
File1=song1.mp3
Title1=Song Title 1
Length1=180
File2=song2.mp3
Title2=Song Title 2
Length2=240
File3=song3.mp3
Title3=Song Title 3
Length3=300
"#;

        let expected_playlist = vec![
            Song {
                file: "song1.mp3".to_string(),
                title: "Song Title 1".to_string(),
                length: Some(180),
            },
            Song {
                file: "song2.mp3".to_string(),
                title: "Song Title 2".to_string(),
                length: Some(240),
            },
            Song {
                file: "song3.mp3".to_string(),
                title: "Song Title 3".to_string(),
                length: Some(300),
            },
        ];

        assert_eq!(playlist_parser::playlist(test_data), Ok(expected_playlist));
    }