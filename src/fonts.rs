use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub fn get(fonts_dir: &Path) {
    let fonts = visit_fonts_path(fonts_dir, &log_files);
    match fonts {
        Ok(v) => v,
        Err(e) => println!("error on listing: {:?}", e)
    }
}

fn log_files(dir: &DirEntry) {
    let mut font_list = Vec::new();
    font_list.push(dir.path().as_os_str().to_string());
    return font_list:Vec<str>;
    // println!("{:?}", dir.path().as_os_str());
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
