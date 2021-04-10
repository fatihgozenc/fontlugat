use cell::Cell;
use druid::Data;
use std::cell;
use std::fs;
use std::sync::Arc;

#[derive(Clone, Data)]
/// The main model for a todo list application.
struct FontList {
    items: Arc<Vec<Font>>,
}

#[derive(Clone, Data)]
/// A single todo item.
pub struct Font {
    id: usize,
    name: String,
    filename: String,
    ext: String,
    path: String,
}

thread_local!(static FONT_ID: Cell<usize> = Cell::new(1));

impl Font {
    pub fn new(file: &walkdir::DirEntry, ext: &str) -> Font {
        return FONT_ID.with(|thread_id| {
            let font_id = thread_id.get();
            thread_id.set(font_id + 1);
            let font_filename_ref = file.file_name().to_str();
            let font_filename = String::from(opt_ref_to_string(font_filename_ref));
            let font_path = file.path().to_str();
            let font_filename_arr: Vec<&str> = font_filename.split(".").collect();
            let font_name = font_filename_arr.first().unwrap().to_string();
            match font_filename {
                _ => {
                    return Font {
                        id: font_id,
                        name: font_name,
                        filename: font_filename,
                        ext: ext.to_string(),
                        path: opt_ref_to_string(font_path),
                    }
                }
            }
        });
    }

    pub fn generate(self) -> [String; 2] {
        let font_css = format!(
            "{start}{name:?};src: local({name:?}), url({path:?}); font-style: normal;font-weight: normal;{end}",
            start = "@font-face{font-family: ",
            name = self.name,
            path = self.path,
            end = "}"
        );
        let font_html = format!(
            "{start}{name}{family}'{name}'{end}",
            start = "<div class=\"fontholder\"><label>",
            family = "</label><input style=\"font-family:",
            name = self.name,
            end = "\" type=\"text\" value=\"The quick brown fox jumps over the lazy dog.\"><input onchange=\"changeSize(this)\" type=\"range\" min=\"4\" max=\"80\" value=\"40\"><span class=\"size\">40px / 30pt / 2.5rem</span></div>"
        );
        // let font_css_buffer = Font::css(font_css);
        // let font_html_buffer = Font::html(font_css);
        let array = [font_css, font_html];
        return array;
    }
}

pub fn opt_ref_to_string(x: Option<&str>) -> String {
    return x.unwrap().to_string();
}
