// use std::fs;
use path::{Path, PathBuf};
use std::ffi::OsStr;
use std::{env, fs, io, path};
use walkdir::WalkDir;

pub fn scan_dir(dir: &String) -> () {
  for entry in WalkDir::new(&dir) {
    match &entry {
      Ok(entry) => match unzip(&entry) {
        0 => {
          println!("{:?} Zipped", &entry)
        }
        1 => {
          println!("Skipped")
        }
        _ => {
          println!("Skipped")
        }
      },
      Err(e) => println!("{:?}", e),
    }
  }
}
