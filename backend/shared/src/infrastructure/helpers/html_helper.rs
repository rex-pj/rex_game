use super::html_helper_trait::HtmlHelperTrait;
use ammonia::clean;
use html2text::from_read;
use std::io::ErrorKind;

#[derive(Clone)]
pub struct HtmlHelper {}

impl HtmlHelper {
    pub fn new() -> Self {
        Self {}
    }
}

impl HtmlHelperTrait for HtmlHelper {
    fn get_plain_text(&self, html: String) -> Result<String, ErrorKind> {
        let plain_text = from_read(html.as_bytes(), 20).map_err(|_| ErrorKind::Other)?;
        Ok(plain_text)
    }

    fn clean_html(&self, html: String) -> Result<String, ErrorKind> {
        let cleaned_html = clean(&html);
        Ok(cleaned_html)
    }
}
