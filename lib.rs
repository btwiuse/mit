//! # mit
//!
//! mit is an MIT{,-0} license generator
//!
//! ```
//! $ mit --help
//! generate MIT{,-0} license
//!
//! Usage: mit [OPTIONS] --author <AUTHOR>
//!
//! Options:
//!   -0, --zero             Use MIT-0 variant
//!   -y, --year <YEAR>      Set year [optional]
//!   -a, --author <AUTHOR>  Set author name
//!   -h, --help             Print help
//!   -V, --version          Print version
//! ```
//!
//! ## Install
//!
//! ```
//! $ cargo install mit
//! ```
//!
//! ## Usage
//!
//! Print license content
//!
//! ```
//! $ mit --author btwiuse
//! MIT License
//!
//! Copyright (c) btwiuse
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
//! Use [MIT-0](https://github.com/aws/mit-0) variant
//!
//! ```
//! $ mit -0 --author btwiuse
//! MIT No Attribution
//!
//! Copyright (c) btwiuse
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy of this
//! software and associated documentation files (the "Software"), to deal in the Software
//! without restriction, including without limitation the rights to use, copy, modify,
//! merge, publish, distribute, sublicense, and/or sell copies of the Software, and to
//! permit persons to whom the Software is furnished to do so.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
//! INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
//! PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
//! HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
//! OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
//! SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//! ```
//!
//! Save to LICENSE file
//!
//! ```
//! $ mit --author btwiuse > LICENSE
//! ```
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
    #[clap(short = 'y', long = "year", value_name = "YEAR", help = "Set year [optional]")]
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
