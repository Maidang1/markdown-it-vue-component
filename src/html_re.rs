use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref attr_name: Regex = Regex::new(r"[a-zA-Z_:][-a-zA-Z0-9_:.]*").unwrap();
    static ref unquoted: Regex = Regex::new(r#"[^"\'=<>`\\x00-\\x20]+"#).unwrap();
    static ref single_quoted: Regex = Regex::new(r#"'[^']*'"#).unwrap();
    static ref double_quoted: Regex = Regex::new(r#""[^"]*""#).unwrap();
    static ref attr_value: Regex = Regex::new(
        format!(
            r#"(?:{}|{}|{})"#,
            unquoted.as_str(),
            single_quoted.as_str(),
            double_quoted.as_str()
        )
        .as_str()
    )
    .unwrap();
    static ref attribute: Regex = Regex::new(
        format!(
            r#"(?:\s+{}(?:\s*=\s*{})?)"#,
            attr_name.as_str(),
            attr_value.as_str()
        )
        .as_str()
    )
    .unwrap();
    static ref open_tag: Regex = Regex::new(
        format!(
            r#"<[A-Za-z][A-Za-z0-9\\-]*{}*\\s*\\/?>"#,
            attr_name.as_str()
        )
        .as_str()
    )
    .unwrap();
    static ref close_tag: Regex = Regex::new(r#"</[A-Za-z][A-Za-z0-9\\-]*\\s*>"#).unwrap();
    static ref comment: Regex = Regex::new(r#"<!---->|<!--(?:-?[^>-])(?:-?[^-])*-->"#).unwrap();
    static ref processing: Regex = Regex::new(r#"<[?][\\s\\S]*?[?]>"#).unwrap();
    static ref declaration: Regex = Regex::new(r#"<![A-Z]+\\s+[^>]*>"#).unwrap();
    static ref cdata: Regex = Regex::new(r#"<!\\[CDATA\\[[\\s\\S]*?\\]\\]>"#).unwrap();
    static ref html_open_close_tag_re: Regex =
        Regex::new(format!("^(?:{}|{})", open_tag.as_str(), close_tag.as_str()).as_str()).unwrap();
    static ref html_self_closing_tag_re: Regex = Regex::new(
        format!(
            r#"^<[A-Za-z][A-Za-z0-9\\-]*{}*\\s*\\/>"#,
            attribute.as_str(),
        )
        .as_str()
    )
    .unwrap();
    static ref html_open_and_close_tag_in_the_same_line_re: Regex = Regex::new(
        format!(
            "^<([A-Za-z][A-Za-z0-9\\-]*){}*\\s*>.*<\\/\\1\\s*>",
            attribute.as_str()
        )
        .as_str()
    )
    .unwrap();
}
