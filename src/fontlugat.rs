// use std::fs;
use path::{Path, PathBuf};
use std::{fs, path};
use walkdir::WalkDir;

/*
pub fn unzip(entry_path: &Path) -> i32 {
    let zip_dir = fs::File::open(entry_path).unwrap();
    // let mut archive = zip::ZipArchive::new(zip_dir).unwrap();
    // for i in 0..archive.len() {
    //     let file = archive.by_index(i).unwrap();
    //     let outpath = match file.enclosed_name() {
    //         Some(path) => {
    //             let test = path.to_owned();
    //             println!("{:?}, ", test);
    //         }
    //         None => continue,
    //     };
    // }
    // {
    //     let comment = file.comment();
    //     if !comment.is_empty() {
    //         println!("File {} comment: {}", i, comment);
    //     }
    // }

    // if (&*file.name()).ends_with('/') {
    //     println!("File {} extracted to \"{}\"", i, outpath.display());
    //     fs::create_dir_all(&outpath).unwrap();
    // } else {
    //     println!(
    //         "File {} extracted to \"{}\" ({} bytes)",
    //         i,
    //         outpath.display(),
    //         file.size()
    //     );
    //     if let Some(p) = outpath.parent() {
    //         if !p.exists() {
    //             fs::create_dir_all(&p).unwrap();
    //         }
    //     }
    //     let mut outfile = fs::File::create(&outpath).unwrap();
    //     io::copy(&mut file, &mut outfile).unwrap();
    // }
    // // Get and Set permissions
    // #[cfg(unix)]
    // {
    //     use std::os::unix::fs::PermissionsExt;

    //     if let Some(mode) = file.unix_mode() {
    //         fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
    //     }
    // }
    return 1;
}
*/

pub fn is_font(x: &str) -> bool {
    match x {
        // Supported formats
        "otf" | "woff" | "woff2" | "eot" | "ttf" => {
            return true;
        }
        _ => {
            return false;
        }
    }
}

pub fn check_extensions(entry: &walkdir::DirEntry) -> i32 {
    let entry_path = Path::new(entry.path());

    match entry_path.extension() {
        Some(ext) => {
            match ext.to_str() {
                // If .zip file return 1
                Some("zip") => {
                    // unzip(entry_path)
                    return -1;
                }
                // if another file
                Some(ext) => {
                    match is_font(ext) {
                        true => {
                            collect_fonts(entry, ext);
                            return 0;
                        }
                        // If not font return -1
                        false => -1,
                    }
                }
                // dir skipped return -1
                _ => {
                    return -1;
                }
            }
        }
        None => {
            // dir skipped return -1
            return -1;
        }
    }
}

pub fn init(dir: &str) -> () {
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
        }
        Err(e) => {
            println!("{:?} doesn't exist.\n\n{:?}", dir, e)
        }
    }
}

pub fn scan_dir(dir: &String) -> () {
    for entry in WalkDir::new(&dir) {
        match &entry {
            Ok(entry) => match check_extensions(&entry) {
                0 => {
                    println!("Font: {:?}", &entry)
                }
                1 => {
                    println!("Zip File: {:?}", &entry)
                }
                _ => continue,
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Debug)]
struct Font {
    id: u32,
    name: String,
    filename: String,
    ext: String,
    path: String,
}

fn opt_ref_to_string(x: Option<&str>) -> String {
    return x.unwrap().to_string();
}

pub fn collect_fonts(file: &walkdir::DirEntry, ext: &str) -> String {
    let font_filename_ref = file.file_name().to_str();
    let font_filename = String::from(opt_ref_to_string(font_filename_ref));
    let font_path = file.path().to_str();
    let font_filename_arr: Vec<&str> = font_filename.split(".").collect();
    let font_name = font_filename_arr.first().unwrap().to_string();
    match font_filename {
        _ => {
            let font = Font {
                id: 0,
                name: font_name,
                filename: font_filename,
                ext: ext.to_string(),
                path: opt_ref_to_string(font_path),
            };
            println!("{:?} ", font);
        }
    }

    // let mut font_styles = "".to_string();
    // let mut font_wrappers = "".to_string();

    // for font in fonts {
    //     // SPLITTING FROM FOLDER PATH
    //     // THEN SPLITTING FROM FONT FORMAT TO GET NAME STR
    //     let font_filename = font.split("/").last().unwrap().split(".").next().unwrap();
    //     let font_style = format!("@font-face{{font-family:{};src:url('{}');}}", font_filename, font);
    //     let font_wrapper = format!(r#"<div class='fontholder'><label>{}</label><input style='font-family:{}' type='text' value='The quick brown fox jumps over the lazy dog.'><input onchange='changeSize(this)' type='range' min='4' max='80' value='40'><span class='size'>40px / 30pt / 2.5rem</span></div>"#, font_filename, font_filename);
    //     font_styles.push_str(&font_style);
    //     font_wrappers.push_str(&font_wrapper);
    // }
    return "hello world".to_string();
}
