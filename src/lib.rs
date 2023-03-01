use core::panic;
use std::{borrow::Cow, path::{PathBuf, Path}, fs::{remove_dir_all, create_dir}, process::Command};

use emojis::get_by_shortcode;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REQUOTE: Regex = Regex::new("\"(.*?)\"").unwrap();
    static ref REEMOJI: Regex = Regex::new(":(.*?):").unwrap();
    pub static ref CONFIG_PATH: PathBuf = home::cargo_home().expect("Couldn't get Cargo home").join("wawatemplating-config");
}

const REPO_URL: &str = "https://github.com/blyxyas/wawatemplating.git";

/// Replace straight quotes (") with curly quotes, U+201C (“) and U+201D (”)
#[inline(always)]
pub fn curly_quotes(content: &str) -> Cow<'_, str> {
    REQUOTE.replace_all(content, "“$1”")
}

/// Replaces all emojicodes (:cat:) to real emojis
#[inline(always)]
pub fn emojis(content: &str) -> String {
    let mut result = <&str>::clone(&content).to_string();
    for cap in REEMOJI.find_iter(content) {
        if let Some(emoji) = get_by_shortcode(&content[cap.start() + 1..cap.end() - 1]) {
            result = content.replace(cap.as_str(), emoji.as_str());
        };
    }
    result
}

#[inline]
pub fn setup() {
	if CONFIG_PATH.exists() {
		return;
	}
	
	create_dir(CONFIG_PATH.as_path()).unwrap_or_else(|e| panic!("Couldn't create directory `{}`: {e}", CONFIG_PATH.display()));

	// Initialize git repo

	Command::new("git").current_dir(CONFIG_PATH.as_path()).args(["init"]).status().expect("Couldn't initialize repository");

	// Add this repo as remote

	Command::new("git").current_dir(CONFIG_PATH.as_path()).args(["remote", "add", "origin", REPO_URL]).status().expect("Couldn't add remote");

	// Enable sparse checkout

	Command::new("git").current_dir(CONFIG_PATH.as_path()).args(["config", "core.sparseCheckout", "true"]).status().expect("Couldn't enable `core.sparseCheckout`");

	// Add templates to sparse-checkout

	std::fs::write(CONFIG_PATH.join(".git").join("info").join("sparse-checkout"), "templates/*").unwrap_or_else(|e| panic!("Couldn't write to {}: {e}", CONFIG_PATH.join(".git").join("info").join("sparse-checkout").display()));

	Command::new("echo").current_dir(CONFIG_PATH.as_path()).args(["templates/*", ">>", ".git/info/sparse-checkout"]).status().expect("Couldn't add templates to `.git/info/sparse-checkout`");

	// Pull

	Command::new("git").current_dir(CONFIG_PATH.as_path()).args(["pull", "--depth=1", "origin", "main"]).status().expect("Couldn't pull");

	println!("WAWATemplating was successfully configured!");
}

#[inline]
pub fn check_for_updates() {
    Command::new("git").current_dir(CONFIG_PATH.as_path()).args(["pull", "--depth=1", "origin", "main", "--rebase"]).status().expect("Couldn't pull");
	
	println!("Repository updated!");
}

#[inline]
pub fn uninstall() {
	let config_path = CONFIG_PATH.as_path();
	if config_path.exists() {
		remove_dir_all(config_path).unwrap_or_else(|e| panic!("Couldn't remove directory {}: {e}", <&Path>::clone(&config_path).display()));
	}
}

pub fn init() {
	create_dir("src").unwrap_or_else(|e| panic!("Couldn't create directory 'src': {e}"));
	
	std::fs::write("wawaconfig.toml", include_bytes!("../wawaconfig.default.toml")).unwrap_or_else(|e| panic!("Couldn't create `wawaconfig.toml`: {e}"));

	std::fs::write("SUMMARY.toml", include_bytes!("../SUMMARY.default.toml")).unwrap_or_else(|e| panic!("Couldn't create `SUMMARY.md`: {e}"));

	std::fs::write("src/introduction.md", include_bytes!("../introduction.default.md")).unwrap_or_else(|e| panic!("Couldn't create `src/introduction.md`: {e}"));
}