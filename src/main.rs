mod gui;
mod lib;

fn main() {
    // lib::run("./fonts");
    gui::init("./src/templates/test.html");
}

// // let fonts = fonts::get(&dir);
// // println!("{:?}", fonts);
// // match fonts::generate_css(fonts){
// //     Ok(res) => res,
// //     Err(err) => println!("HTML compilation error!: {:?}", err),
// // }
// // match open::that(if cfg!(windows){".\\fontlugat.html"} else {"./fontlugat.html"}){
// //     Ok(_res) => println!("Successfully initialized!"),
// //     Err(err) => println!("Browser error!: {:?}", err)
// // }
