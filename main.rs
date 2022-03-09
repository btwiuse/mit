use std::collections::BTreeMap;
use handlebars::Handlebars;

const TEMPLATE: &str = include_str!("./template.hbs");

fn main(){
    let mut handlebars = Handlebars::new();
    assert!(handlebars.register_template_string("template", TEMPLATE).is_ok());
    let mut data = BTreeMap::new();
    data.insert("year".to_string(), "2022".to_string());
    data.insert("fullname".to_string(), "btwiuse".to_string());
    let out = handlebars.render("template", &data).unwrap();
    print!("{}", out);
}
