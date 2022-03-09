use clap::Parser;

fn main() {
    let app = mit::App::parse();
    app.run()
}
