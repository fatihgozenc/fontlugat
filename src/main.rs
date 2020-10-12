mod fonts;
use std::path::Path;

fn main() {
    fonts::get(Path::new("./fonts"));
}
