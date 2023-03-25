use clap::Parser;

fn main() {
    let app = mit::App::parse();
    app.get_matches().is_present("output");
    app.run()
}
