#![warn(clippy::all)]

use clap::Parser as Parse;
use handlebars::no_escape;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use serde_json::json;

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Parse)]
struct Args {
    /// Input directory
    indir: String,
    /// Connection port
    #[arg(long, default_value = "8080")]
    port: u16,
    /// Output directory for HTML files
    #[arg(long, default_value = "./static")]
    outdir: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    init_behaviour: String,
    fail_behaviour: String,
    imports: Vec<String>,
}

mod templating;

fn main() {
    // * Register all templates ====================

    let mut reg = handlebars::Handlebars::new();
    reg.register_escape_fn(no_escape);
    templating::routing_template(&mut reg).expect("Couldn't register `routing.go`");

    // ===========================================

    // * Read configuration ========================

    let mut content = String::new();
    if Path::new("wawaconfig.toml").exists() {
        let mut f = File::open("wawaconfig.toml").expect("Couldn't open `wawaconfig.toml`");
        f.read_to_string(&mut content)
            .expect("Couldn't read configuration `wawaconfig.toml`");
    } else {
        content = include_str!("../wawaconfig.default.toml").to_string();
    }

    let mut config = toml::from_str::<Config>(&content).expect("Couldn't parse configuration");

    for i in 0..config.imports.len() {
        config.imports[i] = format!("\"{}\"", config.imports[i]);
    }

    // * Create www directory ======================

    let args = Args::parse();

    if !Path::new("www").exists() {
        fs::create_dir("www").expect("Couldn't create directory www");
    };

    let mut f =
        File::create("www/routing.go").expect("Couldn't create | open file `www/routing.go`");

    f.write_all(
        reg.render(
            "routing_template",
            &json!({"port": args.port, "directory": args.outdir, "init_behaviour": config.init_behaviour, "fail_behaviour": config.fail_behaviour, "imports": config.imports.join("\n\t")}),
        )
        .expect("Couldn't render `routing.go`")
        .as_bytes(),
    )
    .expect("Couldn't write to file `www/routing.go`");

    // ===========================================

    // * Convert Markdown files to HTML ============

    if !Path::new(&format!("www/{}", &args.outdir)).exists() {
        dbg!("waw");
        fs::create_dir(format!("www/{}", &args.outdir))
            .unwrap_or_else(|_| panic!("Couldn't create directory `{}`", args.outdir));
    }

    let paths = fs::read_dir(&args.indir)
        .unwrap_or_else(|_| panic!("Couldn't read directory `{}`", args.indir));
    for path in paths {
        let path = path.expect("Couldn't process path in input directory");

        if !path.file_name().to_string_lossy().ends_with(".md") {
            continue;
        };

        let content =
            fs::read_to_string(path.path()).expect("Can't get path of file in the input directory");

        let parser = Parser::new_ext(&content, Options::all());

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        let path_filename = path.file_name();
        let filename_str = path_filename.to_string_lossy();

        let mut f = File::create(format!(
            "www/{}/{}.html",
            args.outdir,
            &filename_str[..filename_str.len() - 3]
        ))
        .unwrap_or_else(|_| {
            panic!(
                "Couldn't create / open file `{}`",
                path.file_name().to_string_lossy()
            )
        });

        f.write_all(html_output.as_bytes())
            .expect("Couldn't write to file");
    }

    // ===========================================
}
