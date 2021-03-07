use log::{debug};
use mdbook::preprocess::{Preprocessor, PreprocessorContext, CmdPreprocessor};
use mdbook::book::Book;
use mdbook::errors::Error;
use clap::ArgMatches;
use std::{process, io};
use mdbook::BookItem;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};


#[macro_use]
extern crate lazy_static;


lazy_static! {
    /// if you use mermaid, may be use `Flow1 ==description==> Flow2`, this string will ignore
    static ref RE : Regex= Regex::new(r"==(?P<c>\S+?)==[^>]").unwrap();
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct RegexReplacerItem {
    regex: String,
    rep: String
}

impl Default for RegexReplacerItem {
    fn default() -> Self {
        RegexReplacerItem{
            regex:"".to_string(),
            rep:"".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct RegexReplacerConfigure {
    command: Option<String>,
    items: Option<Vec<RegexReplacerItem>>,
}

impl Default for RegexReplacerConfigure{
    fn default() -> Self {
        RegexReplacerConfigure{command: Option::None, items: Option::None}
    }
}

#[test]
fn test_replace(){
    let c = (Regex::new("==(?P<c>.+?)==").unwrap(), "<mark>$c</mark>".to_string());
    let f = replace_all(&c, "==sasasas== s");
    print!("{}", f);
}

pub fn replace_all(e: & (Regex, String), s: &str) -> String {
    e.0.replace_all(s, e.1.as_str()).into_owned()
}

pub fn handle_each_item(book_item: &mut BookItem, regexes: & Vec<(Regex, String)>) {
    match book_item {
        BookItem::Chapter(chapter) => {

            for e in regexes {
                chapter.content = replace_all(e, chapter.content.as_str());
                debug!("after regex placer: {} => {}:\n{}", e.0, e.1, chapter.content);
            }

            for item in &mut chapter.sub_items {
                handle_each_item(item, regexes);
            }
        }
        _ => {}
    }
}

pub struct RegexReplacerPreprocessor {}

impl Preprocessor for RegexReplacerPreprocessor {

    fn name(&self) -> &str {
        "regex-replacer"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {

        let config = ctx.config.get_preprocessor(self.name()).unwrap();
        let config_string = toml::to_string(config).unwrap();
        let regex_replacer_configure: RegexReplacerConfigure = toml::from_str(config_string.as_str()).unwrap();

        let mut regexes: Vec<(Regex, String)> = Vec::new();

        if let Some(items) = &regex_replacer_configure.items {
            for e in items{
                let regex = Regex::new(e.regex.as_str()).unwrap();
                regexes.push((regex, e.rep.clone()));
            }
        }

        let ii = &mut book.sections;
        for section in ii {
            handle_each_item(section, & regexes);
        }
        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        _renderer == "html"
    }
}

pub fn handle_preprocessor(pre: &dyn Preprocessor) -> Result<(), Error> {
    debug!("mark start");
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        // We should probably use the `semver` crate to check compatibility
        // here...
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;

    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

pub fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(&renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}