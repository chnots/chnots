pub fn web_svg_src(s: Option<String>) -> Option<String> {
    /*     if let Some(s) = s {
        if !s.starts_with("data:image") {
            Some(format!(
                "data:image/svg+xml;base64,{}",
                BASE64_STANDARD.encode(s)
            ))
        } else {
            Some(s)
        }
    } else {
        None
    } */
    s
}
