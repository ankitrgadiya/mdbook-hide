use log::info;
use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use regex::Regex;

pub struct HidePreprocessor {
    hide: bool,
    re: Regex,
}

const NAME: &str = "hide";

impl HidePreprocessor {
    pub fn new(ctx: &PreprocessorContext) -> HidePreprocessor {
        let mut hide = false;
        let re = Regex::new(r"<!--hidden-->").unwrap();

        if let Some(cfg) = ctx.config.get_preprocessor(NAME) {
            if cfg.contains_key("hide") {
                hide = cfg.get("hide").unwrap().as_bool().unwrap();
            }
        }

        HidePreprocessor { hide, re }
    }

    fn process_item(&self, item: BookItem) -> Option<BookItem> {
        if let BookItem::Chapter(ref c) = item {
            if self.re.is_match(&c.content) {
                info!("removing chapter {}", c.name);
                return None;
            }

            let mut new = c.clone();
            new.sub_items.clear();

            for i in c.sub_items.iter() {
                let cloned_item = i.clone();

                if let Some(cloned_item) = self.process_item(cloned_item) {
                    new.sub_items.push(cloned_item)
                }
            }

            return Some(BookItem::Chapter(new));
        } else {
            Some(item)
        }
    }
}

impl Default for HidePreprocessor {
    fn default() -> Self {
        HidePreprocessor {
            hide: false,
            re: Regex::new(r"").unwrap(),
        }
    }
}

impl Preprocessor for HidePreprocessor {
    fn name(&self) -> &str {
        "hide"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        info!("Running mdbook-hide preprocessor");

        if !self.hide {
            return Ok(book);
        }

        let mut updated = Book::new();

        for section in book.sections.iter() {
            let cloned_section = section.clone();

            if let Some(proccessed_item) = self.process_item(cloned_section) {
                updated.push_item(proccessed_item);
            }
        }

        Ok(updated)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
