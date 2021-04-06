// use std::fs;
use path::{Path, PathBuf};
use std::ffi::OsStr;
use std::{env, fs, io, path};
use walkdir::WalkDir;

pub fn unzip(entry: &walkdir::DirEntry) -> i32 {
  let entry = Path::new(entry.path());

  match entry.extension() {
    Some(ext) => {
      match ext.to_str() {
        // If .zip file
        Some("zip") => {
          // Start unzip operation

          let zip_dir = fs::File::open(entry).unwrap();
          let mut archive = zip::ZipArchive::new(zip_dir).unwrap();

          for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            let outpath = match file.enclosed_name() {
              Some(path) => {
                let test = path.to_owned();
                println!("{:?}, ", test);
              }
              None => continue,
            };
            return 0;
            // {
            //   let comment = file.comment();
            //   if !comment.is_empty() {
            //     println!("File {} comment: {}", i, comment);
            //   }
            // }

            // if (&*file.name()).ends_with('/') {
            //   println!("File {} extracted to \"{}\"", i, outpath.display());
            //   fs::create_dir_all(&outpath).unwrap();
            // } else {
            //   println!(
            //     "File {} extracted to \"{}\" ({} bytes)",
            //     i,
            //     outpath.display(),
            //     file.size()
            //   );
            //   if let Some(p) = outpath.parent() {
            //     if !p.exists() {
            //       fs::create_dir_all(&p).unwrap();
            //     }
            //   }
            //   let mut outfile = fs::File::create(&outpath).unwrap();
            //   io::copy(&mut file, &mut outfile).unwrap();
            // }
            // // Get and Set permissions
            // #[cfg(unix)]
            // {
            //   use std::os::unix::fs::PermissionsExt;

            //   if let Some(mode) = file.unix_mode() {
            //     fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            //   }
            // }
          }
          return 0;
        }
        Some("otf") | Some("woff") | Some("woff2") | Some("eot") | Some("ttf") => {
          // Start css generator
          println!("{:?} is a font file.", entry);
          return 1;
        }
        // Else
        _ => {
          println!("Not a font file.");
          return 1;
        }
      }
    }
    None => {
      println!("Directory skipped.");
      return 1;
    }
  }
}
