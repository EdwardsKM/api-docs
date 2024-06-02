pub fn syntax_highlight(json: &str) -> String {
    let mut highlighted = json
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;");

    let re = r#"("(\\u[a-fA-F0-9]{4}|\\[^u]|[^\\"])*"(\\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+-]?\d+)?)"#;

    highlighted = regex::Regex::new(re)
        .unwrap()
        .replace_all(&highlighted, |caps: &regex::Captures| {
            let match_str = caps.get(0).map_or("", |m| m.as_str());
            let cls = if match_str.starts_with('"') {
                if match_str.ends_with(':') {
                    "key"
                } else {
                    "string"
                }
            } else if match_str == "true" || match_str == "false" {
                "boolean"
            } else if match_str == "null" {
                "null"
            } else {
                "number"
            };
            format!("<span class=\"{}\">{}</span>", cls, match_str)
        })
        .into_owned();

    highlighted
}

