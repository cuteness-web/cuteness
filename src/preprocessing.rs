use std::borrow::Cow;

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
	static ref RE: Regex = Regex::new("\"(.*?)\"").unwrap();
}

/// Replace straight quotes (") with curly quotes, U+201C (“) and U+201D (”)
#[inline(always)]
pub(crate) fn replace_to_curly_quotes(content: &str) -> Cow<'_, str> {
	RE.replace_all(content, "“$1”")
}
