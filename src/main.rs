mod fonts;

fn main() {
	let fonts = fonts::get("./fonts");
	for font in fonts {
		println!("Path: {:?}", font);
	}
}
