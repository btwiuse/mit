#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use clap::Parser;
use handlebars::Handlebars;
use std::collections::BTreeMap;

const TEMPLATE: &str = include_str!("./templates/MIT.hbs");
const TEMPLATE_ZERO: &str = include_str!("./templates/MIT-0.hbs");

/// clap command parser struct
#[derive(Debug, Clone, Parser)]
#[clap(
    name = "mit",
    author = "btwiuse <btwiuse@gmail.com>",
    about = "generate MIT{,-0} license",
    version
)]
pub struct App {
    /// use the MIT No Attribution (MIT-0) variant
    #[clap(short = '0', long = "zero", help = "Use MIT-0 variant")]
    pub zero: bool,
    /// year, optional
    #[clap(
        short = 'y',
        long = "year",
        value_name = "YEAR",
        help = "Set year [optional]"
    )]
    pub year: Option<String>,
    /// author name, required
    #[clap(
        short = 'a',
        long = "author",
        value_name = "AUTHOR",
        help = "Set author name"
    )]
    pub author: String,
}

impl App {
    pub fn run(&self) {
        print!("{}", self.to_string());
    }
    pub fn template(&self) -> &str {
        if self.zero {
            TEMPLATE_ZERO
        } else {
            TEMPLATE
        }
    }
    pub fn to_string(&self) -> String {
        let mut handlebars = Handlebars::new();
        assert!(handlebars
            .register_template_string("template", self.template())
            .is_ok());
        let mut data = BTreeMap::new();

        let year = self.year.clone();
        let year = year.unwrap_or("".to_string());
        let author = self.author.clone();
        let year_author = if year.len() == 0 {
            author
        } else {
            format!("{} {}", year, author)
        };
        data.insert("year_author".to_string(), year_author);

        handlebars.render("template", &data).unwrap()
    }
}
