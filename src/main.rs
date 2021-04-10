mod gui;
mod lib;

fn main() {
    lib::run("./fonts");
    gui::init("./src/templates/test.html");
}
