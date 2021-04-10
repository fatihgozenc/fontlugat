use ffi::OsString;
use path::Path;
use std::{ffi, path};
use walkdir::WalkDir;

use super::entry;
use super::font::Font;
// use super::unzip;

pub fn get_fonts(dir: OsString) -> Vec<[String; 2]> {
    let skipped = ["".to_string(), "".to_string()];

    let mut fonts = vec![];
    for entry in WalkDir::new(&dir) {
        match &entry {
            Ok(entry) => match collect(entry) {
                font => {
                    if font != skipped {
                        fonts.push(font);
                    }
                }
            },
            Err(e) => println!("Error: {:?}", e),
        }
    }

    return fonts;
}

fn collect(entry: &walkdir::DirEntry) -> [String; 2] {
    let entry_path = Path::new(entry.path());
    let skipped = ["".to_string(), "".to_string()];
    return match entry_path.extension() {
        Some(ext) => match ext.to_str() {
            // Some("zip") => {
            //     return 1;
            // }
            Some(ext) => match entry::is_font(ext) {
                true => {
                    let font = Font::new(entry, ext);
                    let component = Font::generate(font);
                    return component;
                }
                false => skipped,
            },
            None => skipped,
        },
        None => skipped,
    };
}

// pub fn scan_dir(dir: &String) -> () {
//     for entry in WalkDir::new(&dir) {
//         match &entry {
//             Ok(entry) => println!("Skipped"),
//             Err(e) => return (),
//         }
//     }
// }
