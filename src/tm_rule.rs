use std::collections::HashMap;

use crate::{tm_patterns::TMPatterns, tm_regex::TMRegex};
use serde::Deserialize;

// TODO: implement mutual exclusivity of match & begin/end

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TMRule<'a> {
    name: Option<&'a str>,

    #[serde(rename = "match")]
    match_re: Option<TMRegex>,
    begin: Option<TMRegex>,
    end: Option<TMRegex>,

    #[serde(rename = "contentName")]
    content_name: Option<&'a str>,

    captures: Option<HashMap<u16, TMCapture<'a>>>,
    #[serde(rename = "beginCaptures")]
    begin_captures: Option<HashMap<u16, TMCapture<'a>>>,
    #[serde(rename = "endCaptures")]
    end_captures: Option<HashMap<u16, TMCapture<'a>>>,

    include: Option<&'a str>,

    patterns: Option<TMPatterns<'a>>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TMCapture<'a> {
    name: &'a str,
}
