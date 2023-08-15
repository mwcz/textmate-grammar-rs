/// TextMate grammar deserializer.
///
/// See https://macromates.com/manual/en/language_grammars
mod tm_patterns;
mod tm_regex;
mod tm_rule;

use crate::tm_regex::TMRegex;
use serde::Deserialize;
use std::collections::HashMap;
use tm_patterns::TMPatterns;
use tm_rule::TMRule;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct TMGrammar<'a> {
    #[serde(rename = "scopeName")]
    scope_name: &'a str,
    #[serde(rename = "fileTypes")]
    file_types: Option<Vec<&'a str>>,
    #[serde(rename = "foldingStartMarker")]
    folding_start_marker: Option<TMRegex>,
    #[serde(rename = "foldingStopMarker")]
    folding_stop_marker: Option<TMRegex>,
    patterns: Option<TMPatterns<'a>>,
    #[serde(rename = "firstLineMatch")]
    first_line_match: Option<TMRegex>,

    // TODO: Option< this doesn't adequately capture the valid types for repository
    repository: Option<HashMap<&'a str, TMRule<'a>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asm_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/asm.tmLanguage.json"));

        println!("{asm:#?}");
    }
}
