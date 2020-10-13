mod fonts;

fn main() {
	let fonts = fonts::get("./fonts");
	println!("Result {:?}", fonts);
}
