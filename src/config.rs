use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use toml::Value;

use cuteness::Method;

use crate::cli::Map;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub config: HashMap<String, Value>,
    pub misc: MiscConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MiscConfig {
    /// me when puro
    pub latex: Option<bool>,
    pub html_lang: Option<String>,
    pub additional_html_header: Option<String>,
    pub syntax_highlighting: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PageConfig {
    pub title: String,
    pub pageconf: Option<HashMap<String, Value>>,
    pub additional_css: Option<Vec<String>>,
    #[serde(default)]
    pub method: Method,
    pub params: Option<Vec<Param>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Param {
    pub r#type: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SummaryConfig {
    pub map: Vec<Map>,
}
