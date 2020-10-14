use std::fs;
use std::ffi::OsStr;
use walkdir::WalkDir;

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

pub fn generate(fonts:Vec<String>) -> std::io::Result<()> {
	for font in fonts {
		// SPLITTING FROM FOLDER PATH 
		// THEN SPLITTING FROM FONT FORMAT TO GET NAME STR
		let	font_name = font.split("/").last().unwrap().split(".").next().unwrap();
    let font_css = format!("@font-face{{font-family:{};src:url({});}}", font_name, font);
		println!("Name: {:?}", font_name);
		println!("CSS: {:?}", font_css);
	}

	let html_head = "<!DOCTYPE html><html lang='en'><head><meta charset='UTF-8'><meta name='viewport' content='width=device-width, initial-scale=1.0'><title>Fonts</title></head><body></body></html>";

	//fs::write("fonts.html", template)?;

	Ok(())
}
