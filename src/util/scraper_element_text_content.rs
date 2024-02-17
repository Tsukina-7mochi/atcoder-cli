use scraper::ElementRef;

pub trait TextContent {
    fn text_content(&self) -> String;
}

impl<'a> TextContent for ElementRef<'a> {
    fn text_content(&self) -> String {
        self.text().collect::<Vec<_>>().concat()
    }
}
