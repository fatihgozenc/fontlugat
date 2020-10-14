mod fonts;

fn main() {
	let fonts = fonts::get("./fonts");
	let generate = fonts::generate(fonts);
	match generate {
			Ok(v) => v,
			Err(e) => println!("HTML error! : {:?}", e),
	}
	//open::that("./fonts.html");
}
