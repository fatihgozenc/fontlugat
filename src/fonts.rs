use walkdir::WalkDir;

pub fn get(fonts_path:&str) -> Vec<String> {
	let mut list:Vec<String> = Vec::new();
	for entry in WalkDir::new(fonts_path).into_iter().filter_map(|e| e.ok()) {
		list.push(entry.path().display().to_string());
	}
	return list;
}
