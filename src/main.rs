mod fonts;
use std::env;

fn main() {
    match env::current_dir(){
        Ok(current_path) => {
            let fonts = fonts::get(&current_path.display().to_string());
            match fonts::generate(fonts){
                Ok(res) => res,
                Err(err) => println!("HTML compilation error!: {:?}", err),
            }
            match open::that(if cfg!(windows){".\\fontlugat.html"} else {"./fontlugat.html"}){
                Ok(_res) => println!("Successfully initialized!"),
                Err(err) => println!("Browser error!: {:?}", err)
            }
        }
        Err(error) => println!("has err: {:?}", error)
    }
}
