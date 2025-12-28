use std::io::ErrorKind;

pub trait HtmlHelperTrait {
    fn get_plain_text(&self, html: String) -> Result<String, ErrorKind>;
    fn clean_html(&self, html: String) -> Result<String, ErrorKind>;
}
