mod fonts;
use std::io;

fn main() {
    println!("Enter font folder path: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_n) => {
            let fonts = fonts::get(&input.to_string().trim_end());
            match fonts::generate(fonts){
                Ok(v) => v,
                Err(e) => println!("HTML error! : {:?}", e),
            }
            match open::that("./fonts.html"){
                Ok(v) => println!("{:?}. Look at your browser.", v),
                Err(e) => println!("{:?}", e)
            }
        }
        Err(error) => println!("{:?} has err: {:?}", input, error)
    }
}
