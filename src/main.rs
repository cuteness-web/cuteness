#![warn(clippy::all)]

use clap::Parser as Parse;
use handlebars::no_escape;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use serde_json::json;
use toml::Value;
use wawatemplating::*;
use yaml_front_matter::YamlFrontMatter;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

#[cfg(feature = "sass")]
use std::process::Command;

#[derive(Parse)]
struct Args {
    /// Update the software
    #[command(subcommand)]
    command: Option<SCommand>,
}

#[derive(clap::Subcommand)]
enum SCommand {
    Init,
	Update,
    Setup,
    Build {
        /// Connection port
        #[arg(long, default_value = "8080")]
        port: u16,
        /// Output directory for HTML files
        #[arg(long, default_value = "static")]
        outdir: String,
        /// Command for the sass compiler. E.g. "sass"
        #[cfg(feature = "sass")]
        #[arg(long, default_value = "sass")]
        sassbin: String,
    },
	Uninstall
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    routing: RoutingConfig,
    config: HashMap<String, Value>,
    misc: MiscConfig,
}

#[derive(Serialize, Deserialize, Debug)]
struct RoutingConfig {
    init_behaviour: String,
    fail_behaviour: String,
    imports: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MiscConfig {
    latex: Option<bool>,
    html_lang: Option<String>,
    additional_html_header: Option<String>,
    syntax_highlighting: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct PageConfig {
    title: String,
    subtitle: Option<String>,
    tags: Option<Vec<String>>,
    date: String,
    additional_css: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();
    // * Check for updates =======================

    if !Path::new(CONFIG_PATH.as_path()).exists() {
        setup();
    }

	if let Some(subcommand) = args.command {
		match subcommand {
			SCommand::Build {port, outdir, sassbin} => build(port, outdir, sassbin),
			SCommand::Init => init(),
			SCommand::Update => check_for_updates(),
			SCommand::Uninstall => uninstall(),
			SCommand::Setup => setup(),
			
		}
	}
}

fn build(port: u16, outdir: String, sassbin: String) {
	// * Register all templates ==================

    let mut reg = handlebars::Handlebars::new();
    reg.register_escape_fn(no_escape);
    reg.register_template_file("routing_template", CONFIG_PATH.join("templates/routing.hbs"))
        .expect("Couldn't register `routing.hbs`");
    reg.register_template_file("page_template", CONFIG_PATH.join("templates/page.hbs"))
        .expect("Couldn't register page.hbs");

    // ===========================================

    // * Read configuration ========================

    let mut content = String::new();
    if !Path::new("wawaconfig.toml").exists() {
		panic!("Couldn't find wawaconfig.toml");
	}

	let mut f = File::open("wawaconfig.toml").expect("Couldn't open `wawaconfig.toml`");
	f.read_to_string(&mut content)
		.expect("Couldn't read configuration `wawaconfig.toml`");

    let mut config = toml::from_str::<Config>(&content).expect("Couldn't parse configuration");

    for i in 0..config.routing.imports.len() {
        config.routing.imports[i] = format!("\"{}\"", config.routing.imports[i]);
    }

    // * Create www directory ======================

    if !Path::new("www").exists() {
        fs::create_dir("www").expect("Couldn't create directory www");
    };

    let mut f =
        File::create("www/routing.go").expect("Couldn't create | open file `www/routing.go`");

    f.write_all(
        reg.render(
            "routing_template",
            &json!({"port": port, "directory": outdir, "init_behaviour": config.routing.init_behaviour, "fail_behaviour": config.routing.fail_behaviour, "imports": config.routing.imports.join("\n\t")}),
        )
        .expect("Couldn't render `routing.go`")
        .as_bytes(),
    )
    .expect("Couldn't write to file `www/routing.go`");

    // ===========================================

    // * Create `www` directory and loop each item

    if !Path::new(&format!("www/{}", &outdir)).exists() {
        fs::create_dir(format!("www/{}", &outdir))
            .unwrap_or_else(|e| panic!("Couldn't create directory `{}`: {e}", outdir));
    }

    let paths = fs::read_dir("src")
        .unwrap_or_else(|e| panic!("Couldn't read directory `src`: {e}"));

    for path in paths {
        // * Convert Markdown file to HTML =========

        let path = path.expect("Couldn't process path in input directory");

        if !path.file_name().to_string_lossy().ends_with(".md") {
            continue;
        };

        let content =
            fs::read_to_string(path.path()).expect("Can't get path of file in the input directory");

        let parsed_markdown = YamlFrontMatter::parse::<PageConfig>(&content)
            .expect("Couldn't parse frontmatter metadata");

        let mut binding = curly_quotes(&parsed_markdown.content).to_string();
        binding = emojis(&binding);
        let parser = Parser::new_ext(&binding, Options::all());

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        let path_filename = path.file_name();
        let filename_str = path_filename.to_string_lossy();

        let mut f = File::create(format!(
            "www/{}/{}.html",
            outdir,
            &filename_str[..filename_str.len() - 3]
        ))
        .unwrap_or_else(|e| {
            panic!(
                "Couldn't create / open file `{}`: {e}",
                path.file_name().to_string_lossy()
            )
        });

        // =======================================

        // * Render in-markdown templates (the user can use handlebars even from the files)

        html_output = reg
            .render_template(
                &html_output,
                &json!({"page": &parsed_markdown.metadata, "outer": &config}),
            )
            .expect("Couldn't render unregistered template");

        // =======================================

        // * Render using page's configuration ===

        f.write_if_different(
            reg.render(
                "page_template",
                &json!({
                    "content": html_output,
                    "page": &parsed_markdown.metadata,
                    "misc": &config.misc
                }),
            )
            .unwrap_or_else(|e| {
                panic!(
                    "Couldn't render template for page `{}`: {e}",
                    path.file_name().to_string_lossy()
                )
            })
            .as_bytes(),
            format!(
                "www/{}/{}.html",
                outdir,
                &filename_str[..filename_str.len() - 3]
            ),
        )
        // =======================================
    }

    // ===========================================

    // * Compile styles ==========================

    if Path::new("src/styles").exists() {
        compile_styles(
            &format!("www/{}/styles", &outdir),
            #[cfg(feature = "sass")]
            &sassbin,
        );
    }

    // ===========================================
}

/// As the feature "sass" is enabled, we're going to let Sass take care of the job.
#[cfg(feature = "sass")]
#[cold]
#[inline(never)]
fn compile_styles(outdir: &str, sass_bin: &str) {
    // Compile custom styles
    Command::new(sass_bin)
        .arg(format!("src/styles:{}", &outdir))
        .status()
        .expect("Error while processing path with Sass");
}

/// As the feature "sass" isn't activated, all `.sass` (actually, all not `.css`) files are ignored. `*.css` files are copied to the output directory `styles` subdirectory.
#[cfg(not(feature = "sass"))]
#[cold]
#[inline(never)]
fn compile_styles(indir: &str, outdir: &str) {
    // Just move the files to the new directory
    let paths =
        fs::read_dir("src/styles").unwrap_or_else(|e| panic!("Couldn't open directory {}: {e}", "src"));

    for path in paths {
        let path = path.expect("Couldn't process a path in directory");
        if !Path::new(&outdir).exists() {
            fs::create_dir(&outdir)
                .unwrap_or_else(|e| panic!("Couldn't create directory {}: {e}", &outdir));
        }

        let mut f = File::create(format!(
            "{}/{}",
            &outdir,
            &path.file_name().to_string_lossy()
        ))
        .unwrap_or_else(|e| {
            panic!(
                "Couldn't open file `{}/{}`: {e}",
                &outdir,
                &path.file_name().to_string_lossy()
            )
        });

        if path.file_name().to_string_lossy().ends_with(".css") {
            f.write_if_different(
                fs::read_to_string(path.path())
                    .expect("Couldn't read path")
                    .as_bytes(),
                format!("www/{}", &path.path().to_string_lossy()),
            )
        }
    }
}

/// Write to file ONLY if the contents are different
trait WriteIfDifferent {
    /// Writes
    fn write_if_different<P: AsRef<Path>>(&mut self, buf: &[u8], path: P);
}

impl<W> WriteIfDifferent for W
where
    W: Write,
{
    fn write_if_different<P: AsRef<Path>>(&mut self, buf: &[u8], path: P) {
        // Check hashes

        if !(path.as_ref().exists()
            && blake3::hash(buf)
                == blake3::hash(
                    fs::read_to_string(path)
                        .expect("Couldn't read path")
                        .as_bytes(),
                ))
        {
            self.write_all(buf).expect("Couldn't write to file");
        }
    }
}
