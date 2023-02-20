#![warn(clippy::all)]

use clap::Parser;
use handlebars::no_escape;
use serde::Deserialize;
use serde_json::json;
use toml;

use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "8080")]
    port: u16,
    #[arg(long, default_value = "./static")]
    dir: String,
}

#[derive(Deserialize, Debug)]
struct Config {
	init_behaviour: String,
	fail_behaviour: String,
}

mod templating;

fn main() {
    // Register all templates ====================

    let mut reg = handlebars::Handlebars::new();
	reg.register_escape_fn(no_escape);
    templating::routing_template(&mut reg).expect("Couldn't register `routing.go`");

    // ===========================================

    // Read configuration ========================

	let mut content = String::new();
	if Path::new("wawaconfig.toml").exists() {
		let mut f = File::open("wawaconfig.toml").expect("Couldn't open `wawaconfig.toml`");
		f.read_to_string(&mut content).expect("Couldn't read configuration `wawaconfig.toml`");
	} else {
		content = include_str !("../wawaconfig.default.toml").to_string();
	}

	let config = toml::from_str::<Config>(&content).expect("Couldn't parse configuration");

    // Create www directory ======================

    let args = Args::parse();

    if !Path::new("www").exists() {
        fs::create_dir("www").expect("Couldn't create directory www");
    };

    let mut f =
        File::create("www/routing.go").expect("Couldn't create | open file `www/routing.go`");

    f.write_all(
        reg.render(
            "routing_template",
            &json!({"port": args.port, "directory": args.dir, "init_behaviour": config.init_behaviour, "fail_behaviour": config.fail_behaviour}),
        )
        .expect("Couldn't render `routing.go`")
        .as_bytes(),
    )
    .expect("Couldn't write to file `www/routing.go`");

    // ===========================================
}
