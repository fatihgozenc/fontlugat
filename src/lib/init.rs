// use std::fs;
use path::{Path, PathBuf};
use std::ffi::OsStr;
use std::{env, fs, io, path};
use walkdir::WalkDir;

pub fn init(dir: &str) -> i32 {
  // Getting ABSPATH
  match fs::canonicalize(PathBuf::from(&dir)) {
    Ok(dir) => {
      // Converting PathBuf to OS string
      match dir.into_os_string().into_string() {
        Ok(dir) => {
          // Scan it recursively
          scan_dir(&dir)
        }
        Err(e) => println!("{:?}", e),
      }
      return 0
    }
    Err(e) => {
      println!("{:?} doesn't exist.\n\n{:?}", dir, e)
      return 1
    }
  }
}
