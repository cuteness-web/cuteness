use handlebars;
use serde_json::json;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "8080")]
    port: u16,
    #[arg(long, default_value = "./static")]
    dir: String,
}

mod templating;

fn main() {
    // Register all templates ====================

    let mut reg = handlebars::Handlebars::new();
    templating::routing_template(&mut reg).expect("Couldn't register `routing.go`");

    // ===========================================

    let args = Args::parse();

    if !Path::new("www").exists() {
        fs::create_dir("www").expect("Couldn't create directory www");
    };

	let mut f = File::create("www/routing.go").expect("Couldn't create | open file `www/routing.go`");

	f.write_all(reg.render(
            "routing_template",
            &json!({"port": args.port, "directory": args.dir})
        )
        .expect("Couldn't render `routing.go`").as_bytes()).expect("Couldn't write to file `www/routing.go`");
}
