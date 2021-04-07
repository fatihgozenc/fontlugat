mod Fontlugat;
mod fontlugat;
use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    match argv.len() > 1 {
        true => fontlugat::init(&argv[1]),
        false => {
            usage();
        }
    }
}

fn usage() -> () {
    let command = "\tfontlugat ~/folder/fonts";
    print!("\nUsage: {}\n\n", command);
}

// let fonts = fonts::get(&dir);
// println!("{:?}", fonts);
// match fonts::generate_css(fonts){
//     Ok(res) => res,
//     Err(err) => println!("HTML compilation error!: {:?}", err),
// }
// match open::that(if cfg!(windows){".\\fontlugat.html"} else {"./fontlugat.html"}){
//     Ok(_res) => println!("Successfully initialized!"),
//     Err(err) => println!("Browser error!: {:?}", err)
// }
