use std::collections::HashMap;

use crate::tm_regex::TMRegex;
use serde::Deserialize;

// TODO: implement mutual exclusivity of match & begin/end

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TMRule<'a> {
    name: &'a str,

    #[serde(rename = "match")]
    match_re: TMRegex,
    begin: TMRegex,
    end: TMRegex,

    #[serde(rename = "contentName")]
    content_name: &'a str,

    captures: HashMap<u16, TMCapture<'a>>,
    #[serde(rename = "beginCaptures")]
    begin_captures: HashMap<u16, TMCapture<'a>>,
    #[serde(rename = "endCaptures")]
    end_captures: HashMap<u16, TMCapture<'a>>,

    include: &'a str,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TMCapture<'a> {
    name: &'a str,
}
