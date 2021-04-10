// use druid::{AppLauncher, PlatformError, WindowDesc};
// mod component;
// pub mod gui;

// pub fn init() -> Result<(), PlatformError> {
//     AppLauncher::with_window(WindowDesc::new(gui::build)).launch(())
// }
use std::fs;
use web_view::*;

pub fn init(html_content: &str) {
    return match fs::read_to_string(html_content) {
        Ok(html) => {
            web_view::builder()
                .title("My Project")
                .content(Content::Html(html))
                .size(320, 480)
                .resizable(false)
                .debug(true)
                .user_data(())
                .invoke_handler(|_webview, _arg| Ok(()))
                .run()
                .unwrap();
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };
}
