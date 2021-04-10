use fs::{File, OpenOptions};
use io::prelude::*;
use std::error::Error;
use std::fs;
use std::io;

pub fn css(font_css: String) -> io::Result<()> {
    return writeln!(
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("./src/templates/fonts.css")
            .unwrap(),
        "{}",
        font_css
    );
}

pub fn generate() -> io::Result<()> {
    let mut header: String;

    let head_part = get_part("head");
    let font_css = get_css("fonts");
    let main_css = get_css("main");

    header = head_part
        .replace(
            "<style part='fonts'></style>",
            &format!("<style>{}</style>", font_css),
        )
        .replace(
            "<style part='main'></style>",
            &format!("<style>{}</style>", main_css),
        );

    let font_list = get_part("fonts");
    let footer = get_part("footer");

    return writeln!(
        OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open("./src/templates/test.html")
            .unwrap(),
        "{}",
        format!("{}{}{}", header, font_list, footer)
    );
}

pub fn html(font_html: String) -> io::Result<()> {
    return writeln!(
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("./src/templates/part.fonts.html")
            .unwrap(),
        "{}",
        font_html
    );
}

pub fn get_component(name: &str) -> String {
    return fs::read_to_string(format!("./src/templates/component.{}.html", name))
        .expect("File not found");
}

pub fn get_part(name: &str) -> String {
    return fs::read_to_string(format!("./src/templates/part.{}.html", name))
        .expect("File not found");
}

pub fn get_css(name: &str) -> String {
    return fs::read_to_string(format!("./src/templates/{}.css", name)).expect("File not found");
}
