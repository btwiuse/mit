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
//! print license content
//!
//! ```
//! $ mit --author btwiuse
//! MIT License
//!
//! Copyright (c) 2022 btwiuse
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
//! ```
//!
//! save to [LICENSE](./source/LICENSE) file
//!
//! ```
//! $ mit --author btwiuse > LICENSE
//! ```
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
