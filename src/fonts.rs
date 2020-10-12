use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub fn get(fonts_dir: &Path) {
    visit_fonts_path(fonts_dir, &log_files);
}

fn log_files(dir: &DirEntry) {
    println!("{:?}", dir.path());
}

fn visit_fonts_path(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_fonts_path(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
