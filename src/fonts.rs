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
            let font_path = entry.path().display().to_string();
            if cfg!(windows){
                list.push(font_path.replace("\\", "/"));
            } else {
                list.push(font_path);
            }
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
        let font_name = font.split("/").last().unwrap().split(".").next().unwrap().to_string();
        let font_style = format!("@font-face{{font-family:{};src:url({});}}", font_name, font);
        let font_wrapper = format!("<div class='fontholder'><label>{:?}</label><input style='font-family:{:?}' type='text' value='The quick brown fox jumps over the lazy dog.'><input onchange='changeSize(this)' type='range' min='4' max='80' value='40'><span class='size'>40px / 30pt / 2.5rem</span></div>", font_name, font_name);
        font_styles.push_str(&font_style);
        font_wrappers.push_str(&font_wrapper);
    }

    let html = format!("<!DOCTYPE html><html lang='en'><head><meta charset='UTF-8'><meta name='viewport' content='width=device-width, initial-scale=1.0'><link href='data:image/x-icon;base64,AAABAAEAEBAQAAEABAAoAQAAFgAAACgAAAAQAAAAIAAAAAEABAAAAAAAgAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAY1tYAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEREREREQAAEAAAAAAAAAAAAAAAAAAAABEREREREQAAERERERERAAAREREREREAABERABEREQAAEREAERERAAAREQAREREAABERAAAREQAAEREAABERAAAREQAREREAABERAAABEQAAEREAAAERAAAREREREREAAAEREREREQDgAwAA3/8AAP//AADAAwAAwAMAAMADAADDAwAAwwMAAMMDAADDwwAAw8MAAMMDAADD4wAAw+MAAMADAADgAwAA' rel='icon' type='image/x-icon' /><title>Fontlugat</title><style>*{{-webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:grayscale;box-sizing:border-box}}body{{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI','Roboto','Oxygen','Ubuntu','Cantarell','Fira Sans','Droid Sans','Helvetica Neue',sans-serif;margin:0}}body[theme='dark']{{background:#1E2021;color:#eee}}body[theme='dark'] input{{background:#1E2021;color:#eee}}.fonts{{margin:1% 2%}}header{{align-items:center;display:flex;text-transform:uppercase;letter-spacing:1px;justify-content:space-between;color:#666;background-color:#262A2B;padding:6px 2%}}header svg{{height:40px}}header svg path{{fill:#666}}.fontholder label{{display:block;}}.fontholder{{width:100%;min-height:100px;margin-bottom:1rem;border-bottom:1px solid #666}}.fontholder input[type='range']{{width:300px}}.fontholder input[type='text']{{width:100%;height:100%;font-family:inherit;font-size:40px;height:100px;border:none}}.fontholder input[type='text']:focus{{outline:none;}}.size{{display:block;margin-bottom:1rem;}}{}</style></head><body theme='dark'><header><svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 154.47 40.96'><path d='M6.07,31.5H29a.64.64,0,0,0,.2,0h0a.87.87,0,0,0,.88-.88v-25a.87.87,0,0,0-.88-.88H6.08A4.28,4.28,0,0,0,1.8,9s0,.06,0,.09c0,2.18,0,22.17,0,24.79,0,.06,0,.11,0,.16a4.28,4.28,0,0,0,4,4.26H29.17a.88.88,0,0,0,.88-.88V34.05a.89.89,0,0,0-1.77,0V36.5H6.07a2.5,2.5,0,0,1-2.49-2.4v-.25A2.5,2.5,0,0,1,6.07,31.5ZM10.61,8.76H21.74l.71,5-3-.71h-3.3l-.35.35v1.67l.35.36H18L20,15,20.7,20l-3-.71H16.11l-.33.33v5l.61,1.83H10.61l.61-1.83V10.59Z'/><path d='M46.14,17l.59,4.24-2.53-.59H42.91l-.27.27v4.18l.51,1.52H38.34l.5-1.52V13.38l-.5-1.52H47.6L48.19,16l-2.53-.59H42.91l-.29.3v1.39l.29.3H44.5Z'/><path d='M61.25,14.16V24.33l-.8,1.45L59,26.63H52l-1.46-.85-.8-1.45V14.16l.8-1.46L52,11.86h7l1.46.84Zm-6.58,8.9h1.64l.72-.42.42-.74V16.58L57,15.85l-.72-.43H54.67l-.72.43-.42.73V21.9l.42.74Z'/><path d='M76,26.63h-4.3l-4-7.79-.17-.63h-.08l0,.61,0,6.33.48,1.48H63.11l.5-1.52V13.38l-.5-1.52h4.55l3.8,6.39.17.63h.08l0-.63V13.38l-.51-1.52H76l-.48,1.45.19,12.39Z'/><path d='M85.11,15.64l0,9.47.51,1.52h-5l.5-1.52V15.64l-.27-.28H79.56L77,15.93l.59-4.07h11l.59,4.07-2.53-.57H85.38Z'/><path d='M99.71,22.47l-.59,4.16H90.26l.5-1.52V13.38l-.5-1.52h4.79l-.51,1.52,0,9.43.25.25h2.37Z'/><path d='M112.37,24.12v.21l-.8,1.45-1.46.85h-6.47l-1.46-.85-.8-1.45V13.38l-.51-1.52h4.79l-.51,1.52,0,8.84.29.52.55.32h1.73l.53-.32.29-.52V13.38l-.5-1.52h4.78l-.5,1.52Z'/><path d='M126.12,19.31v5l-.8,1.45-1.45.85h-6.75l-1.46-.85-.8-1.45V14.16l.8-1.46,1.46-.84h7.95l.59,4.15-2.53-.59H119.8l-.72.43-.42.73V21.9l.42.74.72.42h1.39l.72-.42.42-.74V20.68l-2.15.52V17.79h6.45Z'/><path d='M140.94,26.63h-5.21l.48-1.48-.38-1.81-.27-.28-3.17.78-.27.28-.21,1,.48,1.48h-5.21l.93-1.48,2.51-11.81-.49-1.48H138l-.48,1.48L140,25.15ZM134,15.4l-1,4.73.3.29,1.62-.4.19-.19-.93-4.43Z'/><path d='M148.58,15.64l0,9.47.5,1.52h-5l.51-1.52V15.64l-.28-.28H143l-2.53.57.59-4.07h11l.59,4.07-2.53-.57h-1.29Z'/></svg><div class='switcher'><label for='switcher'>Dark</label><input checked onchange='changeTheme()' type='checkbox' name='switcher'></div></header><section class='fonts'>{}</section><script>function changeTheme(){{let theme=document.body.getAttribute('theme');theme=='dark' ? document.body.setAttribute('theme','light') : document.body.setAttribute('theme','dark')}} function changeSize(e){{e.previousElementSibling.style.fontSize=e.value+'px'; e.nextElementSibling.innerText=e.value+'px / '+e.value*0.75+'pt / '+e.value/16+'rem'}}</script></body></html>", font_styles, font_wrappers );
    
    fs::write("fonts.html", html)?;

    Ok(())
}
