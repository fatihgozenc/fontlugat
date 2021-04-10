pub fn is_font(x: &str) -> bool {
    match x {
        // Supported formats
        "otf" | "woff" | "woff2" | "eot" | "ttf" => {
            return true;
        }
        _ => false,
    }
}

pub fn is_zip(x: &str) -> bool {
    match x {
        // Supported formats
        "zip" => {
            return true;
        }
        _ => false,
    }
}
