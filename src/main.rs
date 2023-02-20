use handlebars;
use serde_json::json;

use clap::Parser;

#[derive(Parser)]
struct Args {
	#[arg(long, default_value = "8080")]
	port: u16,
	#[arg(long, default_value = "./static")]
	dir: String
}

mod templating;

fn main() {
	let mut reg = handlebars::Handlebars::new();
	let args = Args::parse();

	templating::routing_template(&mut reg).expect("Couldn't register `routing.go`");
	println!("{}", reg.render("routing_template", &json!({"port": args.port, "directory": args.dir})).expect("Couldn't render `routing.go`"));
}
