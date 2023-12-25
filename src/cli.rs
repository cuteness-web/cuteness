use clap::Parser as Parse;
use serde::{Deserialize, Serialize};

use crate::config::*;

#[derive(Parse)]
pub struct Args {
    /// Update the software
    #[command(subcommand)]
    pub command: Option<SCommand>,
}

#[derive(clap::Subcommand)] //~ ERROR this looks like you are trying to swap `__clap_subcommand` and `clap::Subcommand`
pub enum SCommand {
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


#[derive(Serialize)]
pub struct Page {
    pub config: PageConfig,
    pub path: String,
}


#[derive(Serialize, Deserialize)]
pub struct Map {
    pub title: String,
    pub url: String,
}