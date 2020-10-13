use walkdir::WalkDir;
use std::ffi::OsStr;

pub fn get(fonts_path:&str) -> Vec<String> {

	let mut list:Vec<String> = Vec::new();
	
	for entry in WalkDir::new(fonts_path).into_iter().filter_map(|e| e.ok()) {

		let font_format = match entry.path().extension().and_then(OsStr::to_str) {
			Some("otf") => "otf",
			Some("ttf") => "ttf",
			Some("eot") => "eot",
			Some("woff") => "woff",
			Some("woff2") => "woff2",
			_ => "Not supported",
		};

		if !entry.path().is_dir() && font_format != "Not supported" {
			list.push(entry.path().display().to_string());
		}
	}
	
	return list;
}
