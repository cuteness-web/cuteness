#![warn(clippy::all)]

use anyhow::{bail, Context, Result};
use clap::Parser as Parse;
use cuteness::*;
use handlebars::{handlebars_helper, no_escape};
use hashbrown::HashMap;
use lazy_static::lazy_static;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use serde_json::json;
use toml::Value;
use walkdir::WalkDir;
use yaml_front_matter::YamlFrontMatter;

#[cfg(feature = "tera")]
use tera;

use std::fs::{self, canonicalize, read_dir, read_to_string, File};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Parse)]
struct Args {
    /// Update the software
    #[command(subcommand)]
    command: Option<SCommand>,
}

#[derive(clap::Subcommand)] //~ ERROR this looks like you are trying to swap `__clap_subcommand` and `clap::Subcommand`
enum SCommand {
    /// Builds your `src` directory into `www`
    Build {
        /// Connection port
        #[arg(long, default_value = "8080")]
        port: u16,
        /// Output directory
        #[arg(long, default_value = "www")]
        outdir: String,
        /// Command for the sass compiler. E.g. "sass"
        #[cfg(feature = "sass")]
        #[arg(long, default_value = "sass")]
        sassbin: String,
    },
    /// Initializes the necessary files (configuration, placeholders...), ready to be modified.
    Init,
    /// Updates the internal configuration files in the configuration path; this is an enhanced `git pull`.
    Update,
    /// Creates the necessary configuration directory and its internal files; this is an enhanced `git clone`.
    Setup,
    /// Deletes the `www` directory
    Clean,
    /// Deletes all configuration files. `cargo uninstall` will not remove these, so before using `cargo uninstall`, use this command.
    Uninstall,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    config: HashMap<String, Value>,
    misc: MiscConfig,
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
    pageconf: Option<HashMap<String, Value>>,
    additional_css: Option<Vec<String>>,
    #[serde(default)]
    method: Method,
    params: Option<Vec<Param>>,
}

#[derive(Serialize)]
struct Page {
    config: PageConfig,
    path: String,
}

#[derive(Serialize, Deserialize)]
struct SummaryConfig {
    map: Vec<Map>,
}

#[derive(Serialize, Deserialize)]
struct Map {
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Param {
    r#type: String,
    name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // * Check for updates =======================

    if !Path::new(CONFIG_PATH.as_path()).exists() {
        setup();
    }

    if let Some(subcommand) = args.command {
        match subcommand {
            SCommand::Build {
                port,
                outdir,
                sassbin,
            } => build(port, Path::new(&outdir), sassbin)?,
            SCommand::Init => init(),
            SCommand::Update => check_for_updates(),
            SCommand::Uninstall => uninstall(),
            SCommand::Setup => setup(),
            SCommand::Clean => {
                fs::remove_dir_all("www").context("Couldn't remove directory `www`")?
            }
        }
    }
    Ok(())
}

fn build(port: u16, outdir: &Path, sassbin: String) -> Result<()> {
    // * Register all templates and helpers ======

    dbg!(CONFIG_PATH.display());

    let mut reg = handlebars::Handlebars::new();
    reg.register_escape_fn(no_escape);
    reg.register_template_file(
        "page_template",
        CONFIG_PATH.join("templates").join("page.html.hbs"),
    )
    .context("Couldn't register page.html.hbs")?;
    reg.register_template_file(
        "rocket_routing_template",
        CONFIG_PATH
            .join("templates")
            .join("routing")
            .join("src")
            .join("main.rs.hbs"),
    )
    .context("Couldn't register `templates/routing/src/main.rs.hbs`")?;

    reg.register_template_file(
        "rocket_toml",
        CONFIG_PATH
            .join("templates")
            .join("routing")
            .join("Rocket.toml.hbs"),
    )
    .context("Couldn't register Rocket.toml.hbs")?;

    handlebars_helper!(lower: |method: String| method.to_lowercase());
    reg.register_helper("lower", Box::new(lower));

    handlebars_helper!(file_name: |path: String| {
        let name = Path::new(&path).file_name().unwrap().to_str().unwrap();
        &name[..name.len() - 3]
    });

    reg.register_helper("file_name", Box::new(file_name));

    handlebars_helper!(sanitize: |path: String| {

        lazy_static!{
            static ref RE: regex::Regex = regex::Regex::new("([<>])").unwrap();
        };

        RE.replace_all(&path, "_").to_string()
    });
    reg.register_helper("sanitize", Box::new(sanitize));

    handlebars_helper!(contains: |src: String, search: String| { src.contains(&search)});
    reg.register_helper("contains", Box::new(contains));

    handlebars_helper!(is_pure: |src: String| {
        lazy_static!{
            static ref RE: regex::Regex = regex::Regex::new("([<>])").unwrap();
        };

        !RE.is_match(&src)
    });
    reg.register_helper("is_pure", Box::new(is_pure));

    handlebars_helper!(cut_end: |src: String, to_cut: usize| {
        &src[..src.len() - to_cut]
    });

    reg.register_helper("cut_end", Box::new(cut_end));

    handlebars_helper!(cut_start: |src: String, to_cut: usize| {
        &src[to_cut..]
    });

    reg.register_helper("cut_start", Box::new(cut_start));

    // ===========================================

    // ===========================================

    // * Read configuration ========================

    let mut content = String::new();
    if !Path::new("cuteconfig.toml").exists() {
        panic!("Couldn't find cuteconfig.toml");
    }

    let mut f = File::open("cuteconfig.toml").context("Couldn't open `cuteconfig.toml`")?;
    f.read_to_string(&mut content)
        .context("Couldn't read configuration `cuteconfig.toml`")?;

    let config = toml::from_str::<Config>(&content).context("Couldn't parse configuration")?;

    // * Create output directory ======================

    if !Path::new(outdir).exists() {
        fs::create_dir(outdir)
            .with_context(|| format!("Couldn't create directory {}", outdir.display()))?;
    };

    // ===========================================

    // * Create Cargo project

    let binding = outdir.join("routing");
    let cargo_project = Path::new(&binding);

    {
        let routing_path = Path::new(&outdir).join("routing");
        if !routing_path.exists() {
            fs::create_dir(&routing_path).context("Couldn't create directory `routing`")?;
        };
        if !routing_path.join("src").exists() {
            fs::create_dir(routing_path.join("src"))
                .context("Couldn't create directory `routing/src`")?;
        };
    };

    {
        let mut f = File::create(cargo_project.join("Cargo.toml")).with_context(|| {
            format!(
                "Couldn't create | open file {}/Cargo.toml",
                outdir.display()
            )
        })?;

        f.write_all(
            fs::read_to_string(
                CONFIG_PATH
                    .join("templates")
                    .join("routing")
                    .join("Cargo.toml"),
            )
            .unwrap_or_else(|e| {
                panic!(
                    "Couldn't open file `{}`/templates/routing/Cargo.toml: {e}",
                    CONFIG_PATH.display()
                )
            })
            .as_bytes(),
        )
        .context("Couldn't write to routing file")?;
    }

    let mut f = File::create(cargo_project.join("src").join("main.rs")).with_context(|| {
        format!(
            "Couldn't create | open file {}/src/main.rs",
            outdir.display()
        )
    })?;

    // ===========================================

    // * Generate sidebar from SUMMARY.toml

    if !Path::new("SUMMARY.toml").exists() {
        panic!("Couldn't find SUMMARY.toml");
    }

    let summary: SummaryConfig = toml::from_str(
        &read_to_string("SUMMARY.toml").context("Couldn't get file `SUMMARY.toml`")?,
    )
    .context("Couldn't parse summary in `SUMMARY.toml`")?;

    // ===========================================

    // * Create `www` directory and loop each item

    if !Path::new(&outdir).exists() {
        fs::create_dir(outdir)
            .with_context(|| format!("Couldn't create directory `{}`", outdir.display()))?;
    }

    if !Path::new(&outdir.join("static")).exists() {
        fs::create_dir(&outdir.join("static")).with_context(|| {
            format!(
                "Couldn't create directory `{}`",
                outdir.join("static").display()
            )
        })?;
    }

    // let paths = fs::read_dir("src").context("Couldn't read directory `src`")?;

    let mut pages = Vec::new();

	for path in WalkDir::new("src").into_iter().filter_map(|e| e.ok()) {
        // * Convert Markdown file to HTML =========

        if !path.file_name().to_string_lossy().ends_with(".md") {
            continue;
        };

        let content = fs::read_to_string(path.path())
            .context("Can't get path of file in the input directory")?;

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
            "{}/static/{}.html",
            outdir.display(),
            &filename_str[..filename_str.len() - 3]
        ))
        .with_context(|| {
            format!(
                "Couldn't create / open file `{}/static/{}.html`",
                outdir.display(),
                &filename_str[..filename_str.len() - 3]
            )
        })?;

        // =======================================

        // * Render in-markdown templates (the user can use handlebars even from the files)

        html_output = reg
            .render_template(
                &html_output,
                &json!({"page": &parsed_markdown.metadata, "outer": &config}),
            )
            .context("Couldn't render unregistered template")?;

        // =======================================

        // * Render using page's configuration ===

        f.write_if_different(
            reg.render(
                "page_template",
                &json!({
                "content": html_output,
                "sidebar": summary,
                "page": &parsed_markdown.metadata,
                    "misc": &config.misc
                }),
            )
            .with_context(|| {
                format!(
                    "Couldn't render template for page `{}`",
                    path.file_name().to_string_lossy()
                )
            })?
            .as_bytes(),
            format!(
                "{}/static/{}.html",
                outdir.display(),
                &filename_str[..filename_str.len() - 3]
            ),
        )?;
        // =======================================

        // Throw an error if an unknown property is found
        {
            let params_in_page = params_in_path(path.path());
            if let Some(params) = &parsed_markdown.metadata.params {
                for param in params {
                    if !params_in_page.contains(&param.name) {
                        bail!("Unknown parameter: `{}`", param.name);
                    };
                }
            }
        }

        pages.push(Page {
            config: parsed_markdown.metadata,
            path: path.path().to_string_lossy().to_string(),
        });
    }

    f.write_if_different(
		reg.render(
			"rocket_routing_template",
			&json!({
				"port": port,
				"directory": canonicalize(outdir).context("Couldn't canonicalize output directory")?.join("static"),
				"pages": pages,
				"config_path": CONFIG_PATH.to_string_lossy()
			}),
		).context("Couldn't render `src/main.rs`")?
		.as_bytes(),
		cargo_project.join("src").join("main.rs"))
	.with_context(|| {
		format!(
			"Couldn't create | open file {}",
			cargo_project.join("src").with_file_name("main.rs").display()
		)
	})?;

    let mut f = File::create(cargo_project.join("Rocket.toml")).with_context(|| {
        format!(
            "Couldn't create | open file {}",
            cargo_project.join("Rocket.toml").display()
        )
    })?;

    f.write_if_different(
        reg.render(
            "rocket_toml",
            &json!({
                "config_path": CONFIG_PATH.to_string_lossy()
            }),
        )
        .context("Couldn't render Rocket.toml template (id: `rocket_toml`)")?
        .as_bytes(),
        cargo_project.join("Rocket.toml"),
    )?;

    // Copy 404 page.

    fs::copy(
        CONFIG_PATH.join("templates").join("404.html"),
        outdir.join("static").join("404.html"),
    )
    .context("Couldn't copy 404 page (templates/404.html)")?;

    // ===========================================

    // * Compile styles ==========================

    if Path::new("src/styles").exists() {
        compile_styles(
            &format!("{}/static/styles", &outdir.display()),
            #[cfg(feature = "sass")]
            &sassbin,
        )?;
    }

    if !Path::new(&format!("{}/static/styles", outdir.display())).exists() {
        fs::create_dir(&format!("{}/static/styles", outdir.display())).with_context(|| {
            format!(
                "Couldn't create directory `{}/static/styles`",
                outdir.display()
            )
        })?;
    }

    // * Copy built-in styles ====================

    for file in read_dir(CONFIG_PATH.join("templates").join("styles"))
        .with_context(|| {
            format!(
                "Couldn't get directory {}",
                CONFIG_PATH.join("templates").join("styles").display()
            )
        })?
        .filter_map(|e| e.ok())
    {
        std::fs::copy(
            file.path(),
            format!(
                "{}/static/styles/{}",
                outdir.display(),
                file.file_name().to_string_lossy()
            ),
        )
        .with_context(|| {
            format!(
                "Couldn't copy file `{}` to `{}/static/styles/{}`",
                file.path().display(),
                outdir.display(),
                file.file_name().to_string_lossy()
            )
        })?;
    }

    // ===========================================

    Ok(())
}

/// Write to file ONLY if the contents are different
trait WriteIfDifferent {
    /// Writes
    fn write_if_different<P: AsRef<Path>>(&mut self, buf: &[u8], path: P) -> Result<()>;
}

impl<W> WriteIfDifferent for W
where
    W: Write,
{
    fn write_if_different<P: AsRef<Path>>(&mut self, buf: &[u8], path: P) -> Result<()> {
        // Check hashes

        if !(path.as_ref().exists()
            && blake3::hash(buf)
                == blake3::hash(
                    fs::read_to_string(path)
                        .context("Couldn't read path")?
                        .as_bytes(),
                ))
        {
            self.write_all(buf).context("Couldn't write to file")?;
        }
        Ok(())
    }
}
