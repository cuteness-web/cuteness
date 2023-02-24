use std::borrow::Cow;

use emojis::get_by_shortcode;
use regex::Regex;
use lazy_static::lazy_static;


lazy_static! {
	static ref REQUOTE: Regex = Regex::new("\"(.*?)\"").unwrap();
	static ref REEMOJI: Regex = Regex::new(":(.*?):").unwrap();
}


/// Replace straight quotes (") with curly quotes, U+201C (“) and U+201D (”)
#[inline(always)]
pub	fn curly_quotes(content: &str) -> Cow<'_, str> {
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
	};
	result
}
