// use std::fs;
use path::PathBuf;
use std::{env, fs, path};
mod dir;
mod entry;
mod font;
mod template;
mod unzip;

// pub fn init() {
//     let argv: Vec<String> = env::args().collect();
//     match argv.len() > 1 {
//         true => run(&argv[1]),
//         false => {
//             usage();
//         }
//     }
// }

// fn usage() -> () {
//     let command = "\tfontlugat ~/folder/fonts";
//     print!("\nUsage: {}\n\n", command);
// }

pub fn run(dir: &str) -> () {
    // Getting ABSPATH
    match fs::canonicalize(PathBuf::from(&dir)) {
        Ok(dir) => {
            // Converting PathBuf to OS string
            let fonts: Vec<[String; 2]> = dir::get_fonts(dir.into_os_string());
            for font in fonts {
                template::css(font[0].clone());
                template::html(font[1].clone());
            }
            template::generate();
        }
        Err(e) => {
            println!("{:?} doesn't exist.\n\n{:?}", dir, e)
        }
    }
}
