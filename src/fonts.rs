use std::fs;
use std::ffi::OsStr;
use walkdir::WalkDir;

pub fn get(fonts_path:&str) -> Vec<String> {

    let mut list:Vec<String> = Vec::new();

    for entry in WalkDir::new(fonts_path).into_iter().filter_map(|e| e.ok()) {

        let font_format = match entry.path().extension().and_then(OsStr::to_str) {
            Some("otf") => "otf",
            Some("ttf") => "ttf",
            Some("eot") => "eot",
            Some("woff") => "woff",
            Some("woff2") => "woff2",
            _ => "Not supported",
        };

        if !entry.path().is_dir() && font_format != "Not supported" {
            list.push(entry.path().display().to_string());
        }
    }

    return list;
}

pub fn generate(fonts:Vec<String>) -> std::io::Result<()> {
    let mut font_styles = "".to_string();
    let mut font_wrappers = "".to_string();

    for font in fonts {
        // SPLITTING FROM FOLDER PATH 
        // THEN SPLITTING FROM FONT FORMAT TO GET NAME STR
        let	font_name = font.split("/").last().unwrap().split(".").next().unwrap();
        let font_style = format!("@font-face{{font-family:{};src:url({});}}", font_name, font);
        let font_wrapper = format!("<div class='fontholder'><label>{:?}</label><input style='font-family:{:?}' type='text' value='AaBbCc'><input onchange='changeSize(this)' type='range' min='4' max='80' value='40'><span class='size'>40px / 30pt / 2.5rem</span></div>", font_name, font_name);
        font_styles.push_str(&font_style);
        font_wrappers.push_str(&font_wrapper);
    }

    let html = format!(r#"<!DOCTYPE html><html lang='en'><head><meta charset='UTF-8'><meta name='viewport' content='width=device-width, initial-scale=1.0'><title>Fontlugat</title><style>*{{-webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:grayscale;box-sizing:border-box}}body{{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI','Roboto','Oxygen','Ubuntu','Cantarell','Fira Sans','Droid Sans','Helvetica Neue',sans-serif;margin:0}}body[theme='dark']{{background:#222;color:#eee}}body[theme='dark'] input{{background:#222;color:#eee}}.fonts{{margin:1% 2%}}header{{display:flex;text-transform:uppercase;letter-spacing:1px;justify-content:space-between;color:#fff;background-color:#333;padding:12px 2%}}.fontholder label{{display:block;padding-top:1rem}}.fontholder{{width:100%;min-height:100px}}.fontholder input[type='range']{{width:50%}}.fontholder input[type='text']{{width:100%;height:100%;font-family:inherit;font-size:40px;height:100px;border:none}}.fontholder input[type='text']:focus{{box-sizing:border-box;outline:none;border-bottom:2px solid #333}}{}</style></head><body theme='dark'><header><span>FontLugat</span><div class='switcher'><label for='switcher'>Dark</label><input checked onchange='changeTheme()' type='checkbox' name='switcher'></div></header><section class='fonts'>{}</section><script>function changeTheme(){{let theme=document.body.getAttribute('theme');theme=='dark' ? document.body.setAttribute('theme','light') : document.body.setAttribute('theme','dark')}} function changeSize(e){{e.previousElementSibling.style.fontSize=e.value+'px'; e.nextElementSibling.innerText=e.value+'px / '+e.value*0.75+'pt / '+e.value/16+'rem'}}</script></body></html>"#, font_styles, font_wrappers );
    
    fs::write("fonts.html", html)?;

    Ok(())
}
