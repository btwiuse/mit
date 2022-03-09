//! # mit
//!
//! mit is an MIT license generator
//!
//! ## Install
//!
//! ```
//! $ cargo install mit
//! ```
//!
//! ## Usage
//!
//! ```
//! $ mit --author btwiuse > LICENSE
//! ```
//!
//!
use chrono::Datelike;
use clap::Parser;
use handlebars::Handlebars;
use std::collections::BTreeMap;

const TEMPLATE: &str = include_str!("./template.hbs");

/// clap command parser struct
#[derive(Debug, Clone, Parser)]
#[clap(
    name = "mit",
    author = "btwiuse <btwiuse@gmail.com>",
    about = "generate MIT license",
    version
)]
pub struct App {
    /// year, optional, defaults to current year
    #[clap(short = 'y', long = "year", value_name = "YEAR", help = "Year")]
    pub year: Option<String>,
    /// author name, required
    #[clap(
        short = 'a',
        long = "author",
        value_name = "AUTHOR",
        help = "Author name"
    )]
    pub author: String,
}

fn this_year() -> i32 {
    let current_date = chrono::Utc::now();
    current_date.year()
}

impl App {
    pub fn run(&self) {
        print!("{}", self.to_string());
    }
    pub fn to_string(&self) -> String {
        let mut handlebars = Handlebars::new();
        assert!(handlebars
            .register_template_string("template", TEMPLATE)
            .is_ok());
        let mut data = BTreeMap::new();

        let year = self.year.clone();
        let year = year.unwrap_or(format!("{}", this_year()));
        data.insert("year".to_string(), year);

        let author = self.author.clone();
        data.insert("author".to_string(), author);

        handlebars.render("template", &data).unwrap()
    }
}
